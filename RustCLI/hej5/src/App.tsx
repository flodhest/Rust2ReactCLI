// App.tsx
import React from 'react';
import { Route, Routes } from 'react-router-dom';
import Home from './components/Home';
import PlaceholderComponent1 from './components/PlaceholderComponent1';
import PlaceholderComponent2 from './components/PlaceholderComponent2';

const App: React.FC = () => (
  <Routes>
    <Route path="/" element={<Home />} />
    <Route path="/component1" element={<PlaceholderComponent1 />} />
    <Route path="/component2" element={<PlaceholderComponent2 />} />
  </Routes>
);

export default App;
