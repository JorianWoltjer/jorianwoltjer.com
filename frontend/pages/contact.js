import { Metadata } from "@/components";
import { faHackTheBox } from "@/components/CustomIcons";
import { faDiscord, faGithub, faTwitter, faYoutube } from "@fortawesome/free-brands-svg-icons";
import { faEnvelope, faFlag } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export function CButton({ href, color, icon, size, text }) {
  return <>
    <a href={href}><div className={"c-button " + color}>
      <div className="c-button-icon"><FontAwesomeIcon icon={icon} size={size || "1x"} /></div>
      <div className="c-button-text">{text}</div>
    </div></a>
  </>
}

export default function Contact() {
  return <div className="center">
    <Metadata title="Contact" description="Contact me through any of the following platforms." />
    <h1>Contact</h1>
    <div className="c-buttons">
      <CButton href="https://www.youtube.com/c/J0R1AN/" color="red" icon={faYoutube} text="YouTube" />
      <CButton href="https://ctftime.org/user/83640" color="orange" icon={faFlag} text="CTFtime" />
      <CButton href="https://app.hackthebox.com/profile/178368" color="light-green" icon={faHackTheBox} size="lg" text="HackTheBox" />
      <CButton href="https://twitter.com/J0R1AN" color="blue" icon={faTwitter} text="Twitter" />
      <CButton href="https://discordapp.com/users/298743112421867521" color="discord-blue" icon={faDiscord} text="Discord" />
      <CButton href="https://github.com/JorianWoltjer" color="gray" icon={faGithub} text="GitHub" />
    </div>
    <p className="c-button"><FontAwesomeIcon icon={faEnvelope} /><a href="mailto:contact@jorianwoltjer.com">contact@jorianwoltjer.com</a></p>
  </div>
}