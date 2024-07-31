use optimization::{Func, GradientDescent, Minimizer, NumericalDifferentiation};

pub struct Billiard {
    s_initial: f64,
    s_final: f64,
    number_collisions: i64,
    delta: f64,
}

impl Billiard {
    pub fn new(s_initial: f64, s_final: f64, number_collisions: i64, delta: f64) -> Billiard {
        Billiard {
            s_initial,
            s_final,
            number_collisions,
            delta,
        }
    }

    pub fn s_initial(&self) -> f64 {
        self.s_initial
    }

    pub fn s_final(&self) -> f64 {
        self.s_final
    }

    pub fn num_collisions(&self) -> i64 {
        self.number_collisions
    }

    pub fn z_inside(&self, s: &f64) -> [f64; 2] {
        let z = self.bind_z_inside(self.delta);
        z(s)
    }

    pub fn z_outside(&self, s: &f64) -> [f64; 2] {
        [s.cos(), 0.5 * s.sin()]
    }

    fn bind_z_inside(&self, delta: f64) -> impl Fn(&f64) -> [f64; 2] + '_ {
        move |s: &f64| -> [f64; 2] {
            let normal_vector = [(0.5 * s.cos()), (s.sin())];
            let distance = self.dist(normal_vector, [0.0, 0.0]);
            [
                self.z_outside(s)[0] - normal_vector[0] / distance * delta,
                self.z_outside(s)[1] - normal_vector[1] / distance * delta,
            ]
        }
    }

    // bind_length takes the fixed endpoints and returns the length function.
    // The length function takes a list of N intermediate collisions and
    // and returns the length of the trajectory. The resulting length function
    // is the objective function we are minimizing.
    fn bind_length(&self) -> impl Fn(&[f64]) -> f64 + '_ {
        move |slist: &[f64]| -> f64 {
            let mut sum = self.dist(self.z_outside(&self.s_initial), self.z_inside(&slist[0]));
            for i in 0..slist.len() - 1 {
                if i % 2 == 0 {
                    sum += self.dist(self.z_outside(&slist[i]), self.z_inside(&slist[i + 1]));
                } else {
                    sum += self.dist(self.z_inside(&slist[i]), self.z_outside(&slist[i + 1]));
                }
            }
            if slist.len() % 2 == 0 {
                sum += self.dist(
                    self.z_inside(slist.last().unwrap()),
                    self.z_outside(&self.s_final),
                );
            } else {
                sum += self.dist(
                    self.z_outside(slist.last().unwrap()),
                    self.z_inside(&self.s_final),
                );
            }
            sum
        }
    }

    // the metric in R^2
    fn dist(&self, x: [f64; 2], y: [f64; 2]) -> f64 {
        ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2)).sqrt()
    }

    // equi-partition the interval [s_initial, s_final] into (n+1) intervals.
    // n is the number of intermediate collisions.
    // Returns a list intermediate points.
    // This list is the initial guess for the gradient descent.
    fn srange(&self) -> Vec<f64> {
        let mut slist = Vec::new();
        let delta_s = (self.s_final - self.s_initial) / ((self.number_collisions + 1) as f64);
        for i in 0..self.number_collisions {
            slist.push(self.s_initial + ((i + 1) as f64) * delta_s);
        }

        slist
    }

    // Solve the minimization problem.
    // number_collisions is the number of INTERMEDIATE collisions (not including the fixed endpoints)
    pub fn minimize_length(&self) -> Vec<[f64; 2]> {
        let initial_guess = self.srange();
        let length_function = NumericalDifferentiation::new(Func(self.bind_length()));
        let arg_min_s = GradientDescent::new()
            .minimize(&length_function, initial_guess)
            .position;
        let mut position_vectors: Vec<[f64; 2]> = Vec::new();

        (0..arg_min_s.len()).for_each(|i| {
            if i % 2 == 0 {
                position_vectors.push(self.z_outside(&arg_min_s[i]));
            } else {
                position_vectors.push(self.z_inside(&arg_min_s[i]));
            }
        });

        position_vectors
    }
}

