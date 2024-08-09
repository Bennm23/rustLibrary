use std::{thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

use model::{IMeas, IModel};
use nalgebra::{ArrayStorage, Const, Matrix1, Matrix2, Matrix2x4, Matrix3, Matrix4, Matrix4x2, Matrix6, MatrixN, SMatrix, SVector, Vector, Vector2, Vector3, Vector4};
use rand::{rngs::ThreadRng, thread_rng};
use rand_distr::{num_traits::Pow, Distribution, Normal};

mod model;

type M4D = Matrix4<f64>;
type V4D = Vector4<f64>;
type M3D = Matrix3<f64>;
type V3D = Vector3<f64>;
type M2D = Matrix2<f64>;
type V2D = Vector2<f64>;

fn main() {

    // xyz_cv();
    // xy_cv_2();//TODO Original is better than new
    xy_cv();
}

fn seconds() -> f64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("ERR TIME").as_secs_f64()
}

fn xyz_cv()
{
    let mut rng = thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();

    //Measurement Data
    let truth : V3D = Vector3::new(3000.0, 1000.0, 5000.0);//Measurement
    let vel : V3D = Vector3::new(-12.0, 13.0, 120.0);

    let mut r : M3D = Matrix3::identity();//Measurement Noise Covariance stored in IMeas
    r *= 3.0.pow(2);

    // let mut meas : IMeas<3> = IMeas::<3>::new(r);

    //Measurement frame to state frame matrix, stored in measurement
    // Measurement has dim 3, pass in num states of model
    //Different measurement types must define their own h
    // let h = meas.build_h::<6>();


    //Model Data
    let mut cv : IModel<6> = IModel::<6>::new();
    cv.init_q(0.1);

    //Initialize in model? Impl CV for IModel<6> ?
    cv.p[(0,0)] = 10000.0;
    cv.p[(1,1)] = 10000.0;
    cv.p[(2,2)] = 10000.0;
    cv.p[(3,3)] = 100.0;
    cv.p[(4,4)] = 100.0;
    cv.p[(5,5)] = 100.0;

    println!("CV = {cv}");


    let start_time = seconds();
    let mut last_time = start_time;

    let first = measure_cv::<3>(truth, r, vel, 0.0, &mut rng, dist);
    sleep(Duration::from_millis(500));

    let mut curr_time = seconds();
    let second = measure_cv::<3>(truth, r, vel, curr_time - last_time, &mut rng, dist);

    println!("FIRST = {first} SECOND = {second}");

    //Initialization Step for model backing filter
    cv.x[3] = (second[0] - first[0]) / (curr_time - last_time);
    cv.x[4] = (second[1] - first[1]) / (curr_time - last_time);
    cv.x[5]= (second[2] - first[2]) / (curr_time - last_time);


    loop {
        curr_time = seconds();
        let z = measure_cv::<3>(
            truth, r, vel, curr_time - start_time, &mut rng, dist
        );
        let meas : IMeas<3> = IMeas::<3>::new(r, z);

        let f = cv.f(curr_time - last_time);
        //Predict
        let pred_x = f * cv.x;
        let pred_p = f * cv.p * f.transpose() + cv.q;


        let h = meas.build_h();
        //Update
        let innovation : V3D = z - h * pred_x;
        let innovation_covariance : M3D = h * pred_p * h.transpose() + meas.r;

        //Kalmain gain is of the form to transform the innovation error 
        //into the state frame by applying (uncertainty estimate and innovation)
        let kalman_gain : SMatrix<f64, 6, 3> = pred_p * h.transpose() * innovation_covariance.try_inverse().unwrap();

        cv.x = pred_x + kalman_gain * innovation; 
        cv.p = (Matrix6::identity() - kalman_gain * h) * pred_p;


        let real : V3D = Vector3::new(
            truth[0] + vel[0] * (curr_time - start_time),
            truth[1] + vel[1] * (curr_time - start_time),
            truth[2] + vel[2] * (curr_time - start_time),
        );
        println!("------\nCV After Run = {cv}");

        println!("Real Pos = {real}");

        last_time = curr_time;
        sleep(Duration::from_secs(1));
    }
}

fn measure_cv<const D : usize>(
    truth : SVector<f64, D>,
    noise : SMatrix<f64, D, D>,
    vel   : SVector<f64, D>,
    dt    : f64,
    rng : &mut ThreadRng, dist : Normal<f64>
) -> SVector<f64, D> {

    let mut res : SVector<f64, D> = SVector::<f64, D>::zeros();

    for i in 0 .. D {
        res[i] = truth[i] + vel[i] * dt + (dist.sample(rng) * noise[(i, i)].sqrt());
    }

    res
}
/**
 * XY CV
 */
fn xy_cv() {
    let mut rng = thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();

    let truth : V2D = Vector2::new(3000.0, 1000.0);
    let vel : V2D = Vector2::new(-6.0, 4.5);

    let q : M4D = Matrix4::new(
        0.1, 0.0, 0.0, 0.0,
        0.0, 0.1, 0.0, 0.0,
        0.0, 0.0, 0.1, 0.0,
        0.0, 0.0, 0.0, 0.1
    );
    let mut r : M2D = Matrix2::identity();
    r *= 5.0.pow(2);

    //Transform state estimate to measurement
    let h : SMatrix<f64, 2, 4> = SMatrix::<f64, 2, 4>::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0
    );

    let start_time = seconds();
    let mut last_time = start_time;

    let first = measure_xy_cv(truth, r, vel, 0.0, &mut rng, dist);
    sleep(Duration::from_millis(500));

    let mut curr_time = seconds();
    let second = measure_xy_cv(truth, r, vel, curr_time - last_time, &mut rng, dist);

    // let mut x = Matrix4::new(
    //     second.x, 0.0, 0.0, 0.0,
    //     0.0, second.y, 0.0, 0.0,
    //     0.0, 0.0, (second.x - first .x) / (curr_time - last_time), 0.0,
    //     0.0, 0.0, 0.0, (second.y - first.y) / (curr_time - last_time)
    // );
    let mut x = Vector4::new(
        second.x,
        second.y,
        (second.x - first.x) / (curr_time - last_time),
        (second.y - first.y) / (curr_time - last_time)
    );
    println!("Initial State = {x}");

    last_time = curr_time;

    let mut p : M4D = Matrix4::new(
        10000.0, 0.0, 0.0, 0.0,
        0.0, 10000.0, 0.0, 0.0,
        0.0, 0.0, 100.0, 0.0,
        0.0, 0.0, 0.0, 100.0
    );

    let mut estimations : Vec<V2D> = Vec::new();
    let mut truths : Vec<V2D> = Vec::new();

    loop {
        curr_time = seconds();

        let meas : V2D = measure_xy_cv(truth, r, vel, curr_time - start_time, &mut rng, dist);

        let truth : V2D = Vector2::new(
            truth.x + vel.x * (curr_time - start_time),
            truth.y + vel.y * (curr_time - start_time)
        );

        println!("MEAS = {meas}");
        println!("TRUTH = {truth}");

        let f : M4D = build_xy_cv_f(curr_time - last_time);

        let pred_x : V4D = f * x;
        let pred_p : M4D = f * p * f.transpose() + q;

        let y : V2D = meas - h * pred_x;//The residual of the estimate vs the measurement, in meas frame
        let s_k : M2D = h * pred_p * h.transpose() + r;//Residual of covariances
        let k_k : Matrix4x2<f64>  = pred_p * h.transpose() * s_k.try_inverse().unwrap();

        x = pred_x + k_k * y;
        p = (Matrix4::identity() - k_k * h) * pred_p;

        // let pos = x.fixed_rows::<2>(0);
        // let pos = x.rows_range(2);
        estimations.push(V2D::new(x[0], x[1]));
        truths.push(truth);
        calculate_xy_rmse(&estimations, &truths);


        println!("State Estimate = {x}");
        println!("Error Covariance = {p}");

        last_time = curr_time;

        sleep(Duration::from_secs(1));
    }
}
fn calculate_xy_rmse(estimates : &[V2D], truths : &[V2D]) {
    let n = estimates.len();
    let sum_squared_error : f64 = estimates.iter()
        .zip(truths.iter())
        .map(|(est, truth)| (est - truth).norm_squared())
        .sum();
    let res = (sum_squared_error / n as f64).sqrt();

    println!("RMSE = {res}");
}
fn build_xy_cv_f(dt : f64) -> M4D {
    Matrix4::new(
        1.0, 0.0, dt, 0.0,
        0.0, 1.0, 0.0, dt,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
fn measure_xy_cv(
    truth : V2D, measurement_covariance : M2D,
    vel : V2D, dt : f64,
    rng : &mut ThreadRng, dist : Normal<f64>
) -> V2D {
    
    let x = truth.x + vel.x * dt + (dist.sample(rng) * measurement_covariance[(0, 0)].sqrt());
    let y = truth.y + vel.y * dt + (dist.sample(rng) * measurement_covariance[(1, 1)].sqrt());

    Vector2::new(x, y)
}

/** END XY CV */

/**
 * Constant Position Model for cartesian inputs
 */
fn xyz_cp() {

    let mut rng = thread_rng();
    let normal_dist = Normal::new(0.0, 1.0).unwrap();

    let truth : V3D = Vector3::new(3000.0, 1000.0, -4000.0);

    let q : M3D = Matrix3::new(
        0.01, 0.0, 0.0,
        0.0, 0.01, 0.0,
        0.0, 0.0, 0.01
    );
    let mut r : M3D = Matrix3::identity();
    r = r * 5.0.pow(2);//Assume measurement noise of 5 meters

    let f : M3D = Matrix3::identity();
    let h : M3D = Matrix3::identity();

    let mut x : V3D = measure(truth, r, &mut rng, normal_dist);
    let mut p : M3D = Matrix3::new(
        1000.0, 0.1, 0.1,
        0.1, 1000.0, 0.1,
        0.1, 0.1, 1000.0
    );

    

    loop {
        //TODO: Measure with R, not P
        let meas : V3D = measure(truth, r, &mut rng, normal_dist);

        println!("Meas = {meas}");

        let pred_x = f * x;
        let pred_p = f * p * f.transpose() + q;

        let y_k : V3D = meas - h * pred_x;
        let s_k : M3D = h * pred_p * h.transpose() + r;

        let k_k : M3D = pred_p * h.transpose() * s_k.try_inverse().unwrap();

        x = pred_x + k_k * y_k;
        p = (M3D::identity() - k_k * h) * pred_p;

        println!("State Estimate = {}", x);
        println!("Error Covariance = {}", p);

        sleep(Duration::from_secs(1));
    }

}

fn measure(truth : Vector3<f64>, measurement_covariance : Matrix3<f64>, rng : &mut ThreadRng, dist : Normal<f64>) -> Vector3<f64> {
    
    let x = truth.x + (dist.sample(rng) * measurement_covariance[(0, 0)].sqrt());
    let y = truth.y + (dist.sample(rng) * measurement_covariance[(1, 1)].sqrt());
    let z = truth.z + (dist.sample(rng) * measurement_covariance[(2, 2)].sqrt());

    Vector3::new(x, y, z)
}

fn local() {
    let real = 2500.0;
    let err = 5.0;

    let mut rng = thread_rng();
    let normal_dist = Normal::new(0.0, 1.0).unwrap();


    let mut x = real + (normal_dist.sample(&mut rng) * err); //State estimate
    let mut p = 1.0; // Error Covariance. Represents the uncertainty in the state estimate. Quantifies the confidence in the state estimate

    //Process Noise Covariance. Accounts for the uncertainty of the state transition model. A higher value means the system model is less certain.
    //Represents uncertainty in the process model, i.e, how much the model might deviate from the actual system dynamics. Reflects unpredictability in the state transition
    //Added to P to capture the error induced by the state transition
    let q = 0.3;
    let r = 5.0; // Measurement Noise Covariance. Accounts for the uncertainty of the observations, higher value means the measurements are noisier.

    let f = 1.0; // State Transition Model. Describes how the state evolves from one time step to the next
    let h = 1.0; // Observation Model. Maps the true state space into the observed space.



    loop {
        let measurement = real + (normal_dist.sample(&mut rng) * err);
        println!("Measurement = {measurement}");


        //Predict
        let pred_x = f * x;
        let pred_p = f * p * f + q; //F * P * F^T

        //Update
        let innovation_residual : f64 = measurement - h*pred_x;
        let innovation_covariance : f64 = h * pred_p * h + r;
        let kalman_gain : f64 = pred_p * h * innovation_covariance.pow(-1); //Pk_k-1 * H^T * (H * Pk_k-1 * H^T + R) ^ -1

        x = pred_x + kalman_gain * (innovation_residual);
        p = (1.0 - kalman_gain * h) * pred_p;


        println!("Resulting Estimate = {x}");
        println!("Resulting Error Covariance = {p}");


        sleep(Duration::from_millis(1000));
    }
}


fn xy_cv_2()
{
    let mut rng = thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();

    //Measurement Data
    let truth : V2D = Vector2::new(3000.0, 1000.0);
    let vel : V2D = Vector2::new(-6.0, 4.5);

    let mut r : M2D = Matrix2::identity();//Measurement Noise Covariance stored in IMeas
    r *= 5.0.pow(2);

    //TODO: Analyze error

    // let mut meas : IMeas<3> = IMeas::<3>::new(r);

    //Measurement frame to state frame matrix, stored in measurement
    // Measurement has dim 3, pass in num states of model
    //Different measurement types must define their own h
    // let h = meas.build_h::<6>();


    //Model Data
    let mut cv : IModel<4> = IModel::<4>::new();
    cv.init_q(0.1);

    //Initialize in model? Impl CV for IModel<6> ?
    cv.p[(0,0)] = 10000.0;
    cv.p[(1,1)] = 10000.0;
    cv.p[(2,2)] = 100.0;
    cv.p[(3,3)] = 100.0;

    println!("CV = {cv}");


    let start_time = seconds();
    let mut last_time = start_time;

    let first = measure_cv::<2>(truth, r, vel, 0.0, &mut rng, dist);
    sleep(Duration::from_millis(500));

    let mut curr_time = seconds();
    let second = measure_cv::<2>(truth, r, vel, curr_time - last_time, &mut rng, dist);

    println!("FIRST = {first} SECOND = {second}");

    //Initialization Step for model backing filter
    cv.x[2] = (second[0] - first[0]) / (curr_time - last_time);
    cv.x[3] = (second[1] - first[1]) / (curr_time - last_time);

    let mut estimations : Vec<V2D> = Vec::new();
    let mut truths : Vec<V2D> = Vec::new();


    loop {
        curr_time = seconds();
        let z = measure_cv::<2>(
            truth, r, vel, curr_time - start_time, &mut rng, dist
        );
        let meas : IMeas<2> = IMeas::<2>::new(r, z);

        let f = cv.f(curr_time - last_time);
        //Predict
        let pred_x = f * cv.x;
        let pred_p = f * cv.p * f.transpose() + cv.q;


        let h = meas.build_h();
        //Update
        let innovation : V2D = z - h * pred_x;
        let innovation_covariance : M2D = h * pred_p * h.transpose() + meas.r;

        //Kalmain gain is of the form to transform the innovation error 
        //into the state frame by applying (uncertainty estimate and innovation)
        let kalman_gain : SMatrix<f64, 4, 2> = pred_p * h.transpose() * innovation_covariance.try_inverse().unwrap();

        cv.x = pred_x + kalman_gain * innovation; 
        cv.p = (Matrix4::identity() - kalman_gain * h) * pred_p;


        let real : V2D = Vector2::new(
            truth[0] + vel[0] * (curr_time - start_time),
            truth[1] + vel[1] * (curr_time - start_time),
            // truth[2] + vel[2] * (curr_time - start_time),
        );
        println!("------\nCV After Run = {cv}");

        println!("Real Pos = {real}");

        estimations.push(V2D::new(cv.x[0], cv.x[1]));
        truths.push(real);
        calculate_xy_rmse(&estimations, &truths);

        last_time = curr_time;
        sleep(Duration::from_secs(1));
    }
}