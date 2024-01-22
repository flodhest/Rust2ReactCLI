// App.tsx
import React, { Suspense } from 'react';
import { BrowserRouter as Router, Route, Routes, Outlet } from 'react-router-dom';
import { AppBar, Toolbar, Button, CircularProgress, CssBaseline } from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';
import theme from './Styles/theme';
import './Styles/main.scss';
import Home from './Components/Home/Home';
import PlaceholderComponent1 from './Components/PlaceholderComponent1/PlaceholderComponent1';
import PlaceholderComponent2 from './Components/PlaceholderComponent2/PlaceholderComponent2';
import Link from './Link';

const loading = (
  <div className="pt-3 text-center">
    <CircularProgress />
  </div>
);

const Navbar = () => (
  <AppBar position="static">
    <Toolbar>
      <nav>
        <div>
          <Link to="/">
            <Button color="inherit">Home</Button>
          </Link>
          <Link to="/placeholder/1">
            <Button color="inherit">Placeholder Component 1</Button>
          </Link>
          <Link to="/placeholder/2">
            <Button color="inherit">Placeholder Component 2</Button>
          </Link>
        </div>
      </nav>
    </Toolbar>
  </AppBar>
);

const PlaceholderComponents = () => (
  <Outlet />
);

const App: React.FC = () => (
  <ThemeProvider theme={theme}>
    <CssBaseline />
    <Router>
      <Suspense fallback={loading}>
        <Navbar />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/placeholder/*" element={<PlaceholderComponents />}>
            <Route path="1" element={<PlaceholderComponent1 />} />
            <Route path="2" element={<PlaceholderComponent2 />} />
          </Route>
        </Routes>
      </Suspense>
    </Router>
  </ThemeProvider>
);

export default App;
