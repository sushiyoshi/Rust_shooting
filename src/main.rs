extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston_window::*;
use std::f64;
use std::clone::Clone;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::RenderArgs;
use glutin_window::GlutinWindow as Window;

pub const WIDTH: f64 = 640.0;
pub const HEIGHT: f64 = 480.0;
#[derive(Clone)]
struct Bullet {
    x:f64,
    y:f64,
    size:f64,
    dir:f64,
    speed:f64,
    accele:f64,
    //向きの加速度
    dir_accele:f64,
    color:[f32; 4],
}

struct App {
    bullet:Vec<Bullet>,
    gl: GlGraphics,
    window:Window,
}
//弾を追加するときのオプション
struct Boption {
  rota:f64,
  count:i32,
  st_dir:f64,
}
//ゲームオブジェクトのトレイト
trait Chara {
  fn render(&mut self,args:&RenderArgs,gl:&mut GlGraphics);
  fn update(&mut self);
}
impl App {
  pub fn new() -> App {
    let opengl=OpenGL::V3_2;
    let window = App::new_window(opengl);
    App {
      gl:GlGraphics::new(opengl),
      window: window,
      bullet:Vec::with_capacity(100),
    }
  }
  fn new_window(opengl:OpenGL) -> Window{
    WindowSettings::new(
                "shooting",
                [WIDTH as u32, HEIGHT as u32]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap()  
  }
  fn all(&mut self) {
    let mut events = Events::new(EventSettings::new());
    let mut count = 0.0;
    let mut count_2 =1.0;
    while let Some(e) = events.next(&mut self.window) {
      count += 1.0;
      if count % 20.0 == 0.0 {
        count_2 *= -1.0;
        add(
        vec![Bullet{x:300.0,y:200.0,size:5.0,dir:count*2.0,speed:0.2,accele:0.1,dir_accele:0.5*count_2,color:[1.0,0.0,0.0,1.0]},
        Bullet{x:300.0,y:200.0,size:5.0,dir:count*4.0,speed:0.0,accele:0.2,dir_accele:-1.0*count_2,color:[1.0,0.3,0.8,1.0]}],
        //Default::default(),
        Boption{rota:10.0,count:36,st_dir:count},
        &mut self.bullet);
      }
      if let Some(c) = e.render_args() {
        clear_all(&c);
      }
      //for el in &mut self.bullet {
      let l = self.bullet.len();
      //Rustらしい書き方ではないと思うので要変更
      for i in 1..l+1 {
        //逆順に要素を探索
        let index:usize = l-i;
        let el = &mut self.bullet[index];
        if let Some(r) = e.render_args() {
          el.render(&r,&mut self.gl);
        }
        if let Some(_) = e.update_args() {
          el.update();
        }
        
        if is_in_screen(&el) {
          //println!("delete");
          self.bullet.swap_remove(index);
        }
      }
      //println!("length:{}",self.bullet.len());
    }

  }
}
impl Chara for Bullet {
  fn render(&mut self,args:&RenderArgs,gl:&mut GlGraphics) {
    use graphics::*;
    let cl:[f32; 4]  = self.color;
    let (square,white) = (rectangle::square(0.0, 0.0,self.size),rectangle::square(1.7/10.0*self.size, 1.7/10.0*self.size, self.size/1.7));
    let (x, y) = (self.x,self.y);
    gl.draw(args.viewport(), |c, gl| {
        let transform = c.transform.trans(x, y);
        circle_arc(cl, 5.0,0.0,f64::consts::PI*1.9999,square,transform, gl);
        circle_arc([1.0; 4], 4.0,0.0,f64::consts::PI*1.9999,white,transform, gl);
    });
  }
  fn update(&mut self) {
    let math = f64::consts::PI/180.0 * self.dir;
    self.x += (math.cos() - 0.0)*self.speed;
    self.y += (math.sin() - 0.0)*self.speed;
    self.speed += self.accele;
    self.dir += self.dir_accele;
  }

}
impl Default for Boption {
  fn default() -> Self {
    Self{rota:36.0,count:10,st_dir:0.0}
  }
}
impl Default for Bullet {
  fn default() -> Self {
    Self{x:0.0,
    y:0.0,
    size:5.0,
    dir:0.0,
    speed:1.0,
    accele:0.0,
    //向きの加速度
    dir_accele:0.0,
    color:[1.0; 4],}
  }
}
//画面全消し
fn clear_all(args: &RenderArgs) {
  use graphics::*;
  GlGraphics::new(OpenGL::V3_2).draw(args.viewport(), |c, gl| {
      clear([0.0,0.0,0.0,1.0],gl);
      });
}
//画面内にあるかどうか
fn is_in_screen(bullet:&Bullet) -> bool {
    bullet.y < HEIGHT + bullet.size
    && bullet.y > 12.0-bullet.size
    && bullet.x > WIDTH + bullet.size
    && bullet.x > 12.0-bullet.size
}
//弾を追加する
fn add(obj:Vec<Bullet>,boption:Boption,bullet_data:&mut Vec<Bullet>) {
  let mut d = boption.st_dir;
  for _ in 0..boption.count {
    for e in &obj {
      let mut cp = e.clone();
      cp.dir = d;
      bullet_data.push(cp);
    }
    d+=boption.rota;
  }
}
fn main() {
    let mut bullet_data :Vec<Bullet>;
    let mut app = App::new();
    app.all();
}
