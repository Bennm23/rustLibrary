# Kalman Filtering

## Formula

 ### Predict - a priori

 $$\text{State Estimate}\\
    x^{k-1}_{k} = F x^{k-1}_{k-1} \\
    \text{Error Covariance}
    \\
    p^{k-1}_{k} = F P^{k-1}_{k-1} F^T + Q
 $$

 ### Update

    1. Innovation + prefit residual
    2. Innovation covariance
    3. Optimal Kalman Gain
    4. a posteriori state estimate
    5. a posteriori covariance estimate
    6. Measurement post-fit residual


 $$\begin{array}{ll}
        \text{1. }
        \tilde{y_{k}} = z_{k} - H_k \hat{x}^k_{k-1}\\
        \text{2. }
        S_k = H_kP^k_{k-1}H^T_k + R_k \\
        \text{3. }
        K_k = P^{k}_{k-1} H^T S_k^{-1} \\
        \text{4. }
        \hat{x}_k^k = \hat{x}^k_{k-1} + K_k \tilde{y}_k\\
        \text{5. }
        P_k^k = (I - K_k H_k) P^k_{k-1}\\
        \text{6. }
        \tilde{y}_k^k = z_k - H_k \hat{x}_k^k \\
    \end{array}$$

## Basics

 ### X - State Estimate

 X Defines the State Estimate of the System.

 ### P - Error Covariance

 P Defines the Error Covariance. This encapsulates the uncertainty in the state estimate. It tells us the confidence we have in the state estimate.


 ### Q - Process Noise Covariance

 Q Accounts for the uncertainty of the state transition model. A higher value
 means the system model is less certain. I.e how much the model might deviate
 from the actual system dynamics. Reflects unpredictability in the state transition.

 Added to P to capture the error induced by the state transition.

 "Covariance of the Process Noise"


 ### R - Measurement Noise Covariance

 R Accounts for the uncertainty of the observations, higher values means the
 measurements are noisier.

 If measurements are correlated, you need to include non-zero off-diagonal elements in R to represent the covariance between different measurements.

 "Covariance of the observation noise"

 ### F - State Transition Model

 F describes the transition from the last state to the current state. 

 For models such as a CV, this would be a 6x6 matrix that mutliplies the
 estimate velocities time dt and adds them to the position state. Giving us
 a new state estimate.


 ### H - Observation Model

 H maps the true state space into the observed space.

 **Transforms the State Estimate to the Measurement Frame**
 
 Multiplying by H Transpose maps the uncertainties into the state frame


 H Is not the identity matrix when
  - You are using partial measurements 
  - Measurement is a linear combination (z = x + 2y)
  - Different units or scales (meters to km)
  - When the model involves more comples relationships

 Rows of H: Correspond to the number of measurements
 Cols of H: Correspond to the number of state variables

 General Function
 $$
    z = H x + v
 $$

 - z is the measurement vector
 - H is the observation model matrix
 - x is the state vector
 - v is the measurement noise

### Y - Innovation

 Innovation is the difference between the observed value of a variable at time t
 and the optimal forecast of that value based
 on the information available.

### Z - Measurement

 Z is the measurement at a given time 