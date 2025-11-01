```markdown
# **Front & Rear Perception System**  
## **Unreal Engine 5 – GPU-Accelerated Sensor Suite**  
> **Zero-Copy • Double-Buffered • Real-Time • CycloneDDS • CUDA Graph**

---

## Table of Contents

- [Overview](#overview)
- [Feature Matrix](#feature-matrix)
- [Camera Pipeline – `AFront_camera_publisher` / `ARear_camera_publisher`](#camera-pipeline--afront_camera_publisher--arear_camera_publisher)
  - [Compute Shader Processing (`MainCS.usf`)](#compute-shader-processing-maincsusf)
  - [BMP Mode (`PerceptionMode = 0`)](#bmp-mode-perceptionmode--0)
- [Lidar Pipeline – `AFront_lidar_publisher` / `ARear_lidar_publisher`](#lidar-pipeline--afront_lidar_publisher--arear_lidar_publisher)
  - [`ULidarComponent` – **Core CUDA Engine**](#ulidarcomponent--core-cuda-engine)
- [IMU Pipeline – `UDDSIMUPublisher`](#imu-pipeline--uddsimupublisher)
  - [Key Features](#key-features)
- [ESIM Config System – `UEsimConfigReaderSubsystem`](#esim-config-system--uesimconfigreadersubsystem)
  - [Configuration Structure (Extended)](#configuration-structure-extended)
- [System Components](#system-components)
- [Threading & Async Flow](#threading--async-flow)
- [Data Size (1920×1080)](#data-size-1920×1080)
- [Configuration Panel](#configuration-panel)
- [Debug Output (Toggle On/Off)](#debug-output-toggle-onoff)
- [Quick Start Guide](#quick-start-guide)
- [Performance Highlights](#performance-highlights)

---

<div align="center">

```mermaid
graph TD
    A[Unreal Scene] --> B[Scene Capture]
    B --> C[Front Camera]
    B --> D[Rear Camera]
    B --> E[Front Lidar (5x)]
    B --> F[Rear Lidar (5x)]
    B --> G[Vehicle Pawn]
    C --> H[HLSL Compute Shader]
    D --> H
    E --> I[CUDA Kernel via ULidarComponent]
    F --> I
    G --> J[IMU Publisher]
    H --> K[BMP / BGR8]
    I --> L[Point Cloud]
    J --> M[sensor_msgs::Imu]
    K --> N[CycloneDDS]
    L --> N
    M --> N
```

**High-Performance Dual-Stream Sensor Pipeline**

| **Camera Path** | **Lidar Path** | **IMU Path** |
|-----------------|----------------|-------------|
| **HLSL Compute Shader** | **CUDA via `ULidarComponent`** | **Game Thread + Async DDS** |
| **1 Capture each** | **5 Textures each** | **100 Hz+** |
| **~6.2 MB/frame** | **~1.8 MB/cloud** | **~0.1 KB/msg** |

</div>

---

## Overview

A **fully GPU-accelerated**, **zero-CPU-copy**, **real-time** perception system for **front and rear** vehicle sensors in Unreal Engine 5.  
Includes **HDR camera processing**, **multi-layer Lidar point cloud generation**, and **high-frequency IMU publishing** via **CycloneDDS**.

---

## Feature Matrix

| Feature | **Front Camera** | **Rear Camera** | **Front Lidar** | **Rear Lidar** | **IMU** |
|--------|:----------------:|:---------------:|:---------------:|:--------------:|:------:|
| **Sensor** | `USceneCaptureComponent2D` ×1 | `USceneCaptureComponent2D` ×1 | `USceneCaptureComponent2D` ×5 | `USceneCaptureComponent2D` ×5 | `EsimIMU_1_*` |
| **Processing** | **HLSL Compute Shader** | **HLSL Compute Shader** | **CUDA via `ULidarComponent`** | **CUDA via `ULULidarComponent`** | **Blueprint + C++** |
| **Input** | HDR Float (RGBA32f) | HDR Float (RGBA32f) | Depth, Color, Velocity, Intensity, Mask | Depth, Color, Velocity, Intensity, Mask | Vehicle State |
| **Output** | BMP (with header) or BGR8 | BMP (with header) or BGR8 | ROS2 `PointCloud2` or Custom IDL | ROS2 `PointCloud2` or Custom IDL | `sensor_msgs::Imu` |
| **DDS Topic** | `rt/FrontCameraImageTopic` | `rt/RearCameraImageTopic` | `rt/front_pointcloud` | `rt/rear_pointcloud` | `IMUTopic` |
| **Double Buffer** | Yes (A/B) | Yes (A/B) | Yes (5 pairs) | Yes (5 pairs) | N/A |
| **Motion Blur** | Configurable | Configurable | Always On | Always On | — |
| **Weather Effects** | — | — | Rain/Fog Noise + Dropouts | Rain/Fog Noise + Dropouts | — |
| **Radial Velocity** | — | — | Yes | Yes | Angular & Linear |
| **Debug Save** | BMP/PNG | BMP/PNG | PLY + PNG | PLY + PNG | CSV Log |
| **Mode Switch** | `0`=BMP, `1`=BGR8 | `0`=BMP, `1`=BGR8 | `0`=Custom, `1`=ROS2 | `0`=Custom, `1`=ROS2 | — |
| **Coverage** | 120° FOV | 120° FOV | 120° FOV | 120° FOV | N/A |

---

## Camera Pipeline – `AFront_camera_publisher` / `ARear_camera_publisher`

```mermaid
flowchart LR
    subgraph Capture ["Capture Stage"]
        A[HDR Scene] --> B[USceneCaptureComponent2D]
        B --> C[Double-Buffered RenderTarget A/B]
    end
    subgraph Process ["GPU Processing"]
        C --> D[HLSL Compute Shader]
        D --> E[Tone Map → Gamma → BGR8/BMP]
    end
    subgraph Publish ["Publish Stage"]
        E --> F[CycloneDDS Zero-Copy]
    end
```

### Compute Shader Processing (`MainCS.usf`)

| Step | Operation | Details |
|------|----------|--------|
| 1 | **Sample** | Bilinear HDR (`Texture2D<float4>`) |
| 2 | **Tone Map** | Exposure ×1.5 |
| 3 | **Gamma** | 1/2.2 |
| 4 | **Convert** | `float` → 8-bit BGR |
| 5 | **Error Handling** | NaN/Inf → Black |
| 6 | **BMP Header** | 54-byte (thread 0,0) |
| 7 | **Padding** | 4-byte row alignment |

> **Mode Switch**: `PerceptionMode = 0` → BMP, `1` → BGR8  
> **Zero CPU Copy** via `FRHIGPUBufferReadback`

---

### BMP Mode (`PerceptionMode = 0`)

- Full 54-byte header written on GPU
- Bottom-up row order (flipped Y)
- Row padding to 4-byte boundary
- No extra CPU allocation

---

## Lidar Pipeline – `AFront_lidar_publisher` / `ARear_lidar_publisher`

```mermaid
flowchart LR
    subgraph Capture ["Multi-Texture Capture"]
        A[5x USceneCaptureComponent2D] --> B[Depth, Color, Velocity, Intensity, Mask]
        B --> C[Double-Buffered (5 pairs)]
    end
    subgraph Process ["CUDA Processing"]
        C --> D[ULidarComponent::LaunchPointCloudKernelAsync]
        D --> E[Point Cloud Generation]
        E --> F[CUDA Graph (Optional)]
    end
    subgraph Publish ["DDS Output"]
        F --> G[CycloneDDS Async Publish]
    end
```

### `ULidarComponent` – **Core CUDA Engine**

| Feature | Implementation |
|--------|----------------|
| **Kernel Compilation** | `CompileAndLoadKernel()` via NVRTC + JIT |
| **Kernel Selection** | `depthToPointCloudParentKernelCustomIDL` or `Standard` |
| **External Memory** | D3D12 → CUDA via `cudaImportExternalMemory` |
| **Texture Binding** | `cudaTextureObject_t` (point sampling) |
| **CUDA Graph** | `BeginStreamCapture()` → `cuGraphLaunch` |
| **Pinned Memory** | `cudaHostAlloc` for zero-copy |
| **Sync** | D3D12 Fence → CUDA Semaphore |
| **Compute Support** | Blackwell (`compute_120`) • Ada (`compute_89`) |

> **Runtime Compilation**  
> **Graph Mode** for low-overhead repeated launches

---

## IMU Pipeline – `UDDSIMUPublisher`

```mermaid
flowchart LR
    A[Vehicle Pawn] --> B[EsimIMU_1_* Functions]
    B --> C[LinearAccel, AngularVel, Orientation]
    C --> D[ROS2 sensor_msgs::Imu]
    D --> E[CycloneDDS Writer]
    E --> F[Topic: IMUTopic]
```

### Key Features

| Feature | Value |
|--------|-------|
| **Frame ID** | Configurable (`IMU_Frame`) |
| **Sequence** | Auto-increment |
| **Timestamp** | `sec` + `nanosec` |
| **Covariance** | Configurable diagonal |
| **Log Output** | `Timestamps_XY.log` + `SensorCounts.log` |
| **Rate Logging** | 1 Hz stats |

> **Async DDS Write** via `AsyncTask(ENamedThreads::AnyBackgroundHiPriTask)`

---

## ESIM Config System – `UEsimConfigReaderSubsystem`

```mermaid
flowchart TD
    A[vehicle_config.yml] --> B[Parse YAML]
    B --> C[FEsimData Struct]
    C --> D[Perception Modes]
    C --> E[Spawn Locations]
    C --> F[Camera & Lidar Rig]
    C --> G[IMU Settings]
    D --> H[Front/Rear Sensors]
    E --> I[Spawn in World]
    F --> J[Sensor Placement]
    G --> K[IMU Frame ID, Covariance]
```

### Configuration Structure (Extended)

| Category | Key | Type | Description |
|--------|-----|------|-----------|
| **Perception** | `FrontPerceptionMode` | `int32` | `0`=Off, `1`=On |
| | `RearPerceptionMode` | `int32` | `0`=Off, `1`=On |
| | `PerceptionDefinitionMode` | `int32` | `0`=Custom IDL, `1`=ROS2 |
| | `FlipPointCloudLeftRight` | `bool` | Mirror X-axis |
| **Camera** | `ResolutionX`, `ResolutionY` | `int32` | Image resolution |
| **Motion Blur** | `Enabled` | `bool` | Global motion blur |
| **IMU** | `IMUFrameId` | `FString` | ROS2 frame_id |
| | `OrientationCovarianceDiag` | `FVector` | Diagonal values |
| **Rig (base_link)** | `front_camera_link_loc/rot` | `FVector/FRotator` | Front camera pose |
| | `rear_camera_link_loc/rot` | `FVector/FRotator` | Rear camera pose |
| | `front_laser_link_loc/rot` | `FVector/FRotator` | Front Lidar pose |
| | `rear_laser_link_loc/rot` | `FVector/FRotator` | Rear Lidar pose |

---

## System Components

| Component | Role | Key Functions |
|----------|------|---------------|
| `AFront_camera_publisher` | Front camera | `CaptureFrame`, `ProcessFrameGPU` |
| `ARear_camera_publisher` | Rear camera | `CaptureFrame`, `ProcessFrameGPU` |
| `AFront_lidar_publisher` | Front Lidar | `ProcessLidarFrame`, `PublishPointcloud` |
| `ARear_lidar_publisher` | Rear Lidar | `ProcessLidarFrame`, `PublishPointcloud` |
| `ULidarComponent` | **CUDA Engine** | `CompileAndLoadKernel`, `LaunchPointCloudKernelAsync` |
| `UKernelContainer` | Kernel manager | Stores source & entry points |
| `UDDSIMUPublisher` | **IMU + Stats** | `WriteDDS`, `LogSensorStats` |
| `UEsimConfigReaderSubsystem` | Config loader | YAML → `FEsimData`, spawn handling |

---

## Threading & Async Flow

```mermaid
graph TD
    A[Game Thread] --> B[Timer → Capture]
    B --> C[Render Thread]
    C --> D[ENQUEUE_RENDER_COMMAND]
    D --> E[Scene Capture]
    E --> F[HLSL Shader]
    E --> G[CUDA Stream]
    F --> H[AsyncTask → DDS]
    G --> I[Async Kernel Launch]
    I --> J[High-Priority Task → DDS]
    K[IMU Tick] --> L[Async DDS Write]
```

> **Zero CPU memory copy**  
> **Fully asynchronous**  
> **CUDA Graph support for Lidar**

---

## Data Size (1920×1080)

| Data | Size |
|------|------|
| **BGR8 Image** | **~6.22 MB** |
| **BMP (padded)** | **~6.25 MB** |
| **Point Cloud (per Lidar)** | **~1.84 MB** (76k points × 24 bytes) |
| **Dual Lidar Total** | **~3.68 MB** |
| **IMU Message** | **~100 bytes** |

---

## Configuration Panel

| Setting | Values | Effect |
|--------|--------|-------|
| `FrontPerceptionMode` | `0` = Off, `1` = On | Enable front sensors |
| `RearPerceptionMode` | `0` = Off, `1` = On | Enable rear sensors |
| `PerceptionDefinitionMode` | `0` = Custom, `1` = ROS2 | Output format |
| `FlipPointCloudLeftRight` | `true` / `false` | Mirror X-axis |
| `MotionBlur.Enabled` | `true` / `false` | Add blur |
| `IMUFrameId` | `FString` | ROS2 frame |
| `OrientationCovarianceDiag` | `FVector` | IMU noise model |

---

## Debug Output (Toggle On/Off)

| Output | Format | Folder | Toggle |
|-------|--------|--------|--------|
| Front Camera | BMP/PNG | `SavedImages/` | `bDumpFrontImages` |
| Rear Camera | BMP/PNG | `SavedImages/` | `bDumpRearImages` |
| Front Depth | PNG | `SavedImages/` | `bDumpFrontDepth` |
| Rear Depth | PNG | `SavedImages/` | `bDumpRearDepth` |
| Front Point Cloud | PLY | `PointClouds/` | `bDumpFrontPLY` |
| Rear Point Cloud | PLY | `PointClouds/` | `bDumpRearPLY` |
| **IMU Log** | CSV | `Log/Timestamps_XY.log` | Always |
| **Sensor Rates** | CSV | `Log/SensorCounts.log` | Always |

---

## Quick Start Guide

1. Set `FrontPerceptionMode = 1`, `RearPerceptionMode = 1` in `vehicle_config.yml`  
2. Choose `PerceptionDefinitionMode` (0 or 1)  
3. Set `IMUFrameId` and covariance if needed  
4. Launch in UE5 → Auto-capture + IMU starts  
5. Listen to DDS topics:  
   - `rt/FrontCameraImageTopic`  
   - `rt/RearCameraImageTopic`  
   - `rt/front_pointcloud`  
   - `rt/rear_pointcloud`  
   - `IMUTopic`  
6. View debug in `SavedImages/`, `PointClouds/`, `Log/`

---

## Performance Highlights

| Metric | Value |
|-------|-------|
| **GPU Only** | Yes |
| **CPU Copy** | 0 bytes |
| **Double Buffered** | Yes |
| **Async Publish** | Yes |
| **Real-Time Ready** | Yes |
| **IMU @ 100+ Hz** | Yes |
| **CUDA Graph** | Optional |

---

**Fully GPU-Driven. Zero CPU Bottleneck. Front & Rear Sensor Fusion.**  
*Last Updated: November 2025*

---
```
