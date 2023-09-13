import Particles from "react-tsparticles";
import { loadSlim } from "tsparticles-slim";

async function particlesInit(tsParticles) {
    console.log("init", tsParticles);

    await loadSlim(tsParticles);
}

export default function ParticlesBG() {
    return <Particles id="particles-js" init={particlesInit} options={{
        "particles": {
            "number": {
                "value": 80,
                "density": {
                    "enable": true,
                    "value_area": 800
                }
            },
            "color": {
                "value": "#bde4ff"
            },
            "shape": {
                "type": "circle"
            },
            "opacity": {
                "value": 0.5,
                "random": false,
                "anim": {
                    "enable": false,
                    "speed": 1,
                    "opacity_min": 0.1,
                    "sync": false
                }
            },
            "size": {
                "value": 3,
                "random": true,
                "anim": {
                    "enable": false,
                    "speed": 40,
                    "size_min": 0.1,
                    "sync": false
                }
            },
            "line_linked": {
                "enable": true,
                "distance": 150,
                "color": "#3498db",
                "opacity": 0.6,
                "width": 1
            },
            "move": {
                "enable": true,
                "speed": 1,
                "direction": "none",
                "random": false,
                "straight": false,
                "out_mode": "out",
                "bounce": false,
                "attract": {
                    "enable": false,
                    "rotateX": 600,
                    "rotateY": 1200
                }
            }
        },
        "interactivity": {
            "detect_on": "window",
            "events": {
                "onhover": {
                    "enable": true,
                    "mode": "grab"
                },
                "onclick": {
                    "enable": true,
                    "mode": "push"
                },
                "resize": true
            },
            "modes": {
                "grab": {
                    "distance": 100,
                    "line_linked": {
                        "opacity": 0.5
                    }
                },
                "bubble": {
                    "distance": 150,
                    "size": 10,
                    "duration": 2,
                    "opacity": 0.5,
                    "speed": 3
                },
                "repulse": {
                    "distance": 75,
                    "duration": 0.4
                },
                "push": {
                    "particles_nb": 3
                },
                "remove": {
                    "particles_nb": 2
                }
            }
        },
        "retina_detect": true
    }} />
}