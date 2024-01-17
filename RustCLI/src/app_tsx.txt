// src/App.tsx
import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import PlaceholderComponent1 from './components/PlaceholderComponent1';
import PlaceholderComponent2 from './components/PlaceholderComponent2';
import Home from './components/Home'; 

const App: React.FC = () => {
  return (
    <Router>
      <Switch>
        <Route path="/" exact component={Home} /> 
        <Route path="/component1" component={PlaceholderComponent1} />
        <Route path="/component2" component={PlaceholderComponent2} />

      </Switch>
    </Router>
  );
};

export default App;
