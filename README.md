# VOSP

This project is a solver of the Vlasov Equation built with rust and Tauri. The focus during the development was put on the accessibility and the correctness of the code.

## Numerical methods
The Vlasov equation is an equation used in plasma physics. It is given by the following equation, where "s" denote the species "s". And $\hat C_{s,s'}$ denote the collision operator between species "$s$" and "$s'$".
$$\partial_t f_s + \vec v \partial_x f_s + \frac{q_s}{m_s} \: \vec E \: \partial_v f_s = \sum_{s'} \hat C_{s,s'} $$

The space of the solution lies in the phase space ($\vec x , \vec v$) which means that the space of the solution is at least 2 dimensional, and at best 6 dimensional (3 space, 3 velocity). For the moment, the code only works in 1D-1V.

A solver of the Vlasov-Equation is therefore computationally expensive. Moreover, more often than not there exist a huge difference of kinetics between at least two species. The CFL stability condition is therefore a big problem of the Vlasov-Equation. Hence, the methods described below will not suffer this stability condition. However, the time discretization error is still in $O(\Delta t ^2)$. The user will therefore have to find a compromise between time step and calculation.

### Operator splitting

The operator splitting (or "time splitting" in plasma physics) is a numerical method that transform the equation into simpler problem to solve. 

```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

### Semi-Lagrangian

