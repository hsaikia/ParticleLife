use nannou::prelude::*;

// Num particles per class
const NUM_PARTICLES_PER_CLASS: usize = 500;
const NUM_CLASSES: usize = 5;
const NUM_PARTICLES: usize = NUM_PARTICLES_PER_CLASS * NUM_CLASSES;
const CUT_OFF_RADIUS: f32 = 200.0;
const GRAVITY_MAG_MAX: f32 = 0.1;
const MAX_SPEED: f32 = 2.0;
const MIN_MASS: f32 = 100.0;
const MAX_MASS: f32 = 100.0;
const COLORS : [Srgb<u8>; NUM_CLASSES] = [RED, GREEN, BLUE, ORANGE, YELLOW];

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Particle {
    class : usize,
    pos: Vec2,
    vel: Vec2,
    mass: f32,
}

fn get_rand(a : f32, b : f32) -> f32 {
    (b - a) * random::<f32>() + a
}

impl Particle {
    fn new(class : usize) -> Self {
        Particle {
        class : class,
        pos : Vec2::new(get_rand(-100.0, 100.0), get_rand(-100.0, 100.0)),
        vel : Vec2::new(0.0, 0.0),
        mass : get_rand(MIN_MASS, MAX_MASS),
        }
    }

    fn travel(&mut self, win : &Rect<f32>) {
        self.pos += self.vel;
        if self.pos.x > win.right() || self.pos.x < win.left() {
            self.vel.x *= -1.0;
        }
        if self.pos.y > win.top() || self.pos.y < win.bottom() {
            self.vel.y *= -1.0;
        }
    }

    fn accelerate(&mut self, acc : &Vec2) {
        self.vel += *acc;
        self.vel = self.vel.clamp_length(0.0, MAX_SPEED);
        //println!("Vel {}", self.vel.length());
    }
}

struct Model {
    matrix : Vec<Vec<f32>>,
    particles: Vec<Particle>,
}

fn model(_app: &App) -> Model {
    let mut matrix = Vec::new();
    
    for _ in 0..NUM_CLASSES {
        let mut g_coeffs : Vec<f32> = Vec::new();
        for _ in 0..NUM_CLASSES {
            g_coeffs.push(get_rand(-GRAVITY_MAG_MAX, GRAVITY_MAG_MAX));
        }
        matrix.push(g_coeffs);
    }

    println!("{:?}", matrix);
    
    let mut particles = Vec::new();
    for i in 0..NUM_CLASSES {
        for _ in 0..NUM_PARTICLES_PER_CLASS {
            particles.push(Particle::new(i));
        }
    }
    
    Model {matrix,particles}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    for p in &mut model.particles {
        p.travel(&win);
    }
    
    for i in 0..NUM_PARTICLES {
        let p1 = &model.particles[i];
        let mut acc = Vec2::new(0.0, 0.0);
        for j in 0..NUM_PARTICLES {
            if i == j {
                continue;
            }

            let p2 = &model.particles[j];

            //println!("Interaction on class {} from class {}", p1.class, p2.class);

            let r_vec = p2.pos - p1.pos;
            let r2 = r_vec.length_squared();

            if r2 > CUT_OFF_RADIUS * CUT_OFF_RADIUS {
                continue;
            }

            let g : f32 = model.matrix[p1.class][p2.class];
            //println!("Force on class {} from class {} is {}", p1.class, p2.class, g);

            let acc_mag = (g * p2.mass) / r2.max(0.1);
            //println!("Acc on {} from {} : {}", i, j, acc_mag);
            acc += acc_mag * r_vec.normalize()  
        }
        model.particles[i].accelerate(&acc);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let win = app.window_rect();
    let res_x = win.right() - win.left();
    let res_y = win.top() - win.bottom();

    draw.rect().w_h(res_x, res_y).color(BLACK);

    // Draw a blue ellipse with default size and position.
    for p in &model.particles {
        let sz = p.mass.sqrt();
        draw.ellipse().w(sz).h(sz).xy(p.pos).color(srgba(COLORS[p.class].red, COLORS[p.class].green, COLORS[p.class].blue, 200));
    }
    
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
