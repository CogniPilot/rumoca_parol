model BouncingBall "The classic bouncing ball model"
  parameter Real e=0.8 "Coefficient of restitution";
  parameter Real h0=1.0 "Initial height";
  Real h "Height";
  Real v(start=0.0, fixed=true) "Velocity";
initial equation
  h = h0;
equation
  v = der(h);
  der(v) = -9.81;
  reinit(v, -e*pre(v));
end BouncingBall;