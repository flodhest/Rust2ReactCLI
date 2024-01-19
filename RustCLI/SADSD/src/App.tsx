// App.tsx
import React from 'react';
import { Route, Switch } from 'react-router-dom';
import Home from './components/Home';
import PlaceholderComponent1 from './components/PlaceholderComponent1';
import PlaceholderComponent2 from './components/PlaceholderComponent2';

const App: React.FC = () => (
  <Switch>
    <Route path="/" exact component={Home} />
    <Route path="/component1" component={PlaceholderComponent1} />
    <Route path="/component2" component={PlaceholderComponent2} />
  </Switch>
);

export default App;
