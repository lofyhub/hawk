use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SuccessData<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse<T> {
    success: bool,
    data: SuccessData<T>,
}

impl<T> SuccessResponse<T> {
    pub fn new_single(data: T) -> Self {
        SuccessResponse {
            success: true,
            data: SuccessData::Single(data),
        }
    }
    pub fn new_multiple(data: Vec<T>) -> Self {
        SuccessResponse {
            success: true,
            data: SuccessData::Multiple(data),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    success: bool,
    error: T,
}
impl<T> ErrorResponse<T> {
    pub fn new(data: T) -> Self {
        ErrorResponse {
            success: false,
            error: data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthCheck {
    pub status: &'static str,
    pub uptime: &'static str,
    pub message: &'static str,
}

pub const HEALTH_CHECKS: [HealthCheck; 10] = [
    HealthCheck {
        status: "I'm alive and caffeinated! ☕",
        uptime: "Still going strong, just like your coffee addiction!",
        message: "Everything's brewing nicely!",
    },
    HealthCheck {
        status: "All systems are a-go! 🚀",
        uptime: "Longer than your latest binge-watch session!",
        message: "To infinity and beyond!",
    },
    HealthCheck {
        status: "Feeling fresh as a daisy! 🌼",
        uptime: "You can't keep a good server down!",
        message: "All green lights here, boss!",
    },
    HealthCheck {
        status: "Still kicking like a kangaroo! 🦘",
        uptime: "Running smoother than a baby's bottom!",
        message: "Ready to hop into action!",
    },
    HealthCheck {
        status: "Online and loving it! 💻",
        uptime: "I've been up longer than your last relationship. 😜",
        message: "Keep calm and carry on, I'm all good!",
    },
    HealthCheck {
        status: "Running like a cheetah on espresso! 🐆",
        uptime: "Faster than your morning routine!",
        message: "Speed and stability, that's my game!",
    },
    HealthCheck {
        status: "Alive and well in cyberspace! 🌐",
        uptime: "More reliable than your Wi-Fi. 😎",
        message: "Everything's looking great from here!",
    },
    HealthCheck {
        status: "Fit as a fiddle and ready to serve! 🎻",
        uptime: "Outlasting your favorite sneakers!",
        message: "No bugs, just hugs! 🤗",
    },
    HealthCheck {
        status: "All systems are purring like a kitten! 🐱",
        uptime: "Longer than your last nap. 😴",
        message: "No worries, I'm on it!",
    },
    HealthCheck {
        status: "Up and at 'em! 🌞",
        uptime: "I haven't slept, but I feel great!",
        message: "Running smoother than butter on hot toast!",
    },
];
