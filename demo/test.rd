import "std.time" as time;
import "std.window" as win;

const S_WIDTH: uint = 650;
const S_HEIGHT: uint = 400;


struct Ball {
    x: float,
    y: float,
    r: float,
    xs: float,
    ys: float
}

impl Ball {
    fun constructor(){
        self.x = S_WIDTH as float / 2f;
        self.y = S_HEIGHT as float / 2f;
        self.r = 5f;
        self.xs = 1f;
        self.ys = 0f;
    }
    fun move(self) {
        self.x += self.xs;
        self.y += self.ys;
    }
    fun draw(self) {
        // neco
    }
}

enum Sides {
    Left = 50,
    Right = 600
}

struct Player {
    side: Sides,
    y: float,
    w: float,
    h: float,
    speed: float,
    points: uint
}

impl Player {
    fun constructor(side: Sides){
        self.y = 0f;
        self.w = 20f;
        self.h = 100f;
        self.speed = 1f;
        self.points = 0;
        self.side = side;
    }
    fun move(self, direction: float) {
        self.y += self.speed * direction;
        if self.y < 0 {
            self.y = 0;
        }
        else if self.y > S_HEIGHT - self.h {
            self.y = S_HEIGHT - self.h;
        }
        if self.x < 0 {
            self.x = 0;
        }
        if self.x > S_HEIGHT - self.h {
            self.x = S_HEIGHT - self.h;
        }
    }
}

fun main(){
    let ctx = win.init();
    sdfa("dgfg", 60 + 8) as integer + (60f) as float - [60 + 3; 60];
}
