import React from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { Link as MuiLink, Typography } from '@mui/material';

const Link: React.FC<{ to: string; className?: string; children: React.ReactNode }> = ({ to, className, children }) => (
  <Typography component="div" className={className}>
    <MuiLink component={RouterLink} to={to} color="inherit">
      {children}
    </MuiLink>
  </Typography>
);

export default Link;
