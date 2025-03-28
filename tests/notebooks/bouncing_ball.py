"""
Automatically generated by Rumoca
"""
import sympy
import numpy as np
import scipy.integrate

cos = sympy.cos
sin = sympy.sin
tan = sympy.tan

class Model:
    """
    Flattened Modelica Model
    """

    def __init__(self):
        # ============================================
        # Initialize
        self.solved = False

        # ============================================
        # Declare time
        self.time = sympy.symbols('time')

        # ============================================
        # Declare u
        self.u = sympy.Matrix([])
        self.u0 = { }

        # ============================================
        # Declare p
        e = sympy.symbols('e')
        h0 = sympy.symbols('h0')
        self.p = sympy.Matrix([
            e, 
            h0])
        self.p0 = { 
            'e': 0.8, 
            'h0': 1.0}

        # ============================================
        # Declare cp
        self.cp = sympy.Matrix([])
        self.cp0 = { }

        # ============================================
        # Declare x
        h = sympy.symbols('h')
        v = sympy.symbols('v')
        self.x = sympy.Matrix([
            h, 
            v])
        self.x0 = { 
            'h': 1.0, 
            'v': 0.0}

        # ============================================
        # Declare m
        self.m = sympy.Matrix([])
        self.m0 = { }

        # ============================================
        # Declare y
        z = sympy.symbols('z')
        self.y = sympy.Matrix([
            z])
        self.y0 = { 
            'z': 0.0}

        # ============================================
        # Declare z
        self.z = sympy.Matrix([])
        self.z0 = { }

        
        # ============================================
        # Declare pre_x
        pre_h = sympy.symbols('pre_h')
        pre_v = sympy.symbols('pre_v')
        self.pre_x = sympy.Matrix([
            pre_h, 
            pre_v])

        # ============================================
        # Declare pre_m
        self.pre_m = sympy.Matrix([])

        # ============================================
        # Declare pre_z
        self.pre_z = sympy.Matrix([])

        # ============================================
        # Declare x_dot
        der_h = sympy.symbols('der_h')
        der_v = sympy.symbols('der_v')
        self.x_dot = sympy.Matrix([
            der_h, 
            der_v])

        # ============================================
        # Define Continous Update Function: fx
        self.fx = sympy.Matrix([
            z - (2 * h + v), 
            v - (der_h), 
            der_v - (-9.81)])

        # ============================================
        # Define Conditions: c
        self.c = { 
            '__c0': h < 0 }

        # ============================================
        # Define Reset Functions: fr
        def __fr___c0(x):
            pre_h, pre_v= self.x
            h, v= self.x
            v = -e * pre_v
            return [
            h, 
            v]
        self.fr___c0 = sympy.lambdify([self.x, self.p], __fr___c0(self.x))

        # ============================================
        # Events and Event callbacks
        self.zc___c0 = sympy.lambdify([self.time, self.x], h - 0)
        self.zc___c0.terminal = True


    def solve(self):
        # ============================================
        # Solve for explicit ODE
        v = sympy.Matrix(list(self.x_dot) + list(self.y))
        sol = sympy.solve(self.fx, v)
        self.sol_x_dot = self.x_dot.subs(sol)
        self.sol_y = self.y.subs(sol)
        self.f_x_dot = sympy.lambdify([self.time, self.x, self.m, self.u, self.p], list(self.sol_x_dot))
        self.f_y = sympy.lambdify([self.time, self.x, self.m, self.u, self.p], list(self.sol_y))
        self.solved = True

    def __repr__(self):
        return repr(self.__dict__)

    def simulate(self, t0, tf, dt, f_u=None, max_events=100):
        """
        Simulate the modelica model
        """
        if not self.solved:
            self.solve()
        
        if f_u is None:
            def f_u(t):
                return np.zeros(self.u.shape[0])

        # ============================================
        # Declare initial vectors
        u0 = np.array([self.u0[k] for k in self.u0.keys()])
        p0 = np.array([self.p0[k] for k in self.p0.keys()])
        cp0 = np.array([self.cp0[k] for k in self.cp0.keys()])
        x0 = np.array([self.x0[k] for k in self.x0.keys()])
        m0 = np.array([self.m0[k] for k in self.m0.keys()])
        y0 = np.array([self.y0[k] for k in self.y0.keys()])
        z0 = np.array([self.z0[k] for k in self.z0.keys()])
        
        # ============================================
        # Declare Events
        events = [
            self.zc___c0
        ]

        event_callback = {
            0: lambda t, x: self.fr___c0(x, p0),
        }

        # ============================================
        # Solve IVP
        event_count = 0
        t1 = tf
        data = {
            't': [],
            'x': [],
            'u': [],
            'y': [],
        }

        while t0 < tf - dt and event_count < max_events:
            t_eval = np.arange(t0, tf, dt)
            res = scipy.integrate.solve_ivp(
                y0=x0,
                fun=lambda ti, x: self.f_x_dot(ti, x, m0, f_u(ti), p0),
                t_span=[t_eval[0], t_eval[-1]],
                t_eval=t_eval,
                events=events,
            )

            # check for event
            x1 = res['y'][:, -1]
            t1 = res['t'][-1]
            if res.t_events is not None:
                event_count += 1
                for i, t_event in enumerate(res.t_events):
                    if len(t_event) > 0:
                        if i in event_callback:
                            x1 = event_callback[i](t_event[i], x1)

            x = res['y']
            t = res['t']
            u = np.array([ f_u(ti) for ti in t ]).T
            y = np.array([ self.f_y(ti, xi, m0, ui, p0) for (ti, xi, ui) in zip(t, x.T, u.T) ]).T

            data['x'].append(x)
            data['t'].append(t)
            data['u'].append(u)
            data['y'].append(y)

            t0 = t1
            x0 = x1
        
        for k in data.keys():
            if len(data[k]) > 0:
                data[k] = np.hstack(data[k])
        return data
