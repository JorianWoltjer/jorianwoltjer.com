import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faYoutube, faTwitter, faDiscord, faGithub } from "@fortawesome/free-brands-svg-icons";
import { faEnvelope, faFlag } from "@fortawesome/free-solid-svg-icons";
import Image from "next/image";

export default function Contact() {
    return <div className="center">
        <style jsx>{`
            .fa-smaller {
                font-size: 95%;
            }
            .fa-hackthebox {
                width: 1.2em;
                height: 1.2em;
                transform: translateX(0.2em);
            }
        `}</style>
        <h1>Contact</h1>
        <div className="c-buttons">
            <a href="https://www.youtube.com/c/J0R1AN/" target="_blank"><div className="c-button red">
                <div className="c-button-icon"><FontAwesomeIcon icon={faYoutube} /></div>
                <div className="c-button-text">YouTube</div>
            </div></a>
            <a href="https://ctftime.org/user/83640" target="_blank"><div className="c-button orange">
                <div className="c-button-icon"><FontAwesomeIcon className="fa-smaller" icon={faFlag} /></div>
                <div className="c-button-text">CTFtime</div>
            </div></a>
            <a href="https://app.hackthebox.com/profile/178368" target="_blank"><div className="c-button light-green">
                <div className="c-button-icon"><div className="fa-hackthebox"><Image fill src="/img/hackthebox.svg" alt="Logo" /></div></div>
                <div className="c-button-text">HackTheBox</div>
            </div></a>
            <a href="https://twitter.com/J0R1AN" target="_blank"><div className="c-button blue">
                <div className="c-button-icon"><FontAwesomeIcon icon={faTwitter} /></div>
                <div className="c-button-text">Twitter</div>
            </div></a>
            <a href="https://discordapp.com/users/298743112421867521" target="_blank"><div className="c-button discord-blue">
                <div className="c-button-icon"><FontAwesomeIcon className="fa-smaller" icon={faDiscord} /></div>
                <div className="c-button-text">Discord</div>
            </div></a>
            <a href="https://github.com/JorianWoltjer" target="_blank"><div className="c-button gray">
                <div className="c-button-icon"><FontAwesomeIcon icon={faGithub} /></div>
                <div className="c-button-text">GitHub</div>
            </div></a>
        </div>
        <p className="c-button"><FontAwesomeIcon icon={faEnvelope} /><a href="mailto: contact@jorianwoltjer.com">contact@jorianwoltjer.com</a></p>
    </div>
}