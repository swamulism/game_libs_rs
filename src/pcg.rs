use noise::{BasicMulti, Billow, Checkerboard, Cylinders, Fbm, HybridMulti, NoiseFn, OpenSimplex,
            Perlin, RidgedMulti, Seedable, SuperSimplex, Value, Worley};
use rand::{thread_rng, Rng};

const STEP_SIZE: f64 = 0.1;
const NUM_SQUARES: usize = 100;

pub fn get_noises() -> Vec<(String, Vec<Vec<f64>>)> {
    let perlin = generate_perlin(NUM_SQUARES, STEP_SIZE);
    let worley = generate_worley(NUM_SQUARES, STEP_SIZE);
    let open_simplex = generate_open_simplex(NUM_SQUARES, STEP_SIZE);
    let super_simplex = generate_super_simplex(NUM_SQUARES, STEP_SIZE);
    let basic_multi = generate_basic_multi(NUM_SQUARES, STEP_SIZE);
    let billow = generate_billow(NUM_SQUARES, STEP_SIZE);
    let checkerboard = generate_checkerboard(NUM_SQUARES, STEP_SIZE);
    let cylinders = generate_cylinders(NUM_SQUARES, STEP_SIZE);
    let fbm = generate_fbm(NUM_SQUARES, STEP_SIZE);
    let hybrid_multi = generate_hybrid_multi(NUM_SQUARES, STEP_SIZE);
    let ridged_multi = generate_ridged_multi(NUM_SQUARES, STEP_SIZE);
    let value = generate_value(NUM_SQUARES, STEP_SIZE);

    let noises = vec![
        ("perlin".to_string(), perlin),
        ("worley".to_string(), worley),
        ("open_simplex".to_string(), open_simplex),
        ("super_simplex".to_string(), super_simplex),
        ("basic_multi".to_string(), basic_multi),
        ("billow".to_string(), billow),
        ("checkerboard".to_string(), checkerboard),
        ("cylinders".to_string(), cylinders),
        ("fbm".to_string(), fbm),
        ("hybrid_multi".to_string(), hybrid_multi),
        ("ridged_multi".to_string(), ridged_multi),
        ("value".to_string(), value),
    ];
    noises
}

fn generate_perlin(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Perlin::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_super_simplex(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = SuperSimplex::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_open_simplex(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = OpenSimplex::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_basic_multi(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = BasicMulti::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_billow(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Billow::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_checkerboard(size: usize, step: f64) -> Vec<Vec<f64>> {
    let noise = Checkerboard::new();

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_cylinders(size: usize, step: f64) -> Vec<Vec<f64>> {
    let noise = Cylinders::new();

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_fbm(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Fbm::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

/// TODO: fix values going above 1.0 and below 0.0
fn generate_hybrid_multi(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = HybridMulti::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_ridged_multi(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = RidgedMulti::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

/// TODO: fix values going above 1.0 and below 0.0
fn generate_value(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Value::new().set_seed(rng.gen::<u32>());

    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_worley(size: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Worley::new().set_seed(rng.gen::<u32>()).enable_range(true);
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; size]; size];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}
