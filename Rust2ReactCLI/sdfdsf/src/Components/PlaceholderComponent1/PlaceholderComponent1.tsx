import React from 'react';
import Link from '../../Link';
import './PlaceholderComponent1.scss'; 

const PlaceholderComponent1: React.FC = () => (
  <div>
    <h1>Placeholder Component 1</h1>
    <p>This is a placeholder component. You can customize its content and functionality.</p>
    <Link to="/">Go to Home</Link>
  </div>
);

export default PlaceholderComponent1;
