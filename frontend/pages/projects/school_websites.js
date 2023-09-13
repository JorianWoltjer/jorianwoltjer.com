import { Metadata } from "@/components";
import { CButton } from "@/pages/contact";
import { faBook, faCalendarAlt, faChalkboard, faClock, faGraduationCap, faHome, faPoll } from "@fortawesome/free-solid-svg-icons";
import { useEffect } from "react";

export default function SchoolWebsites() {
    useEffect(() => {
        // Keyboard shortcuts
        document.addEventListener("keydown", (e) => {
            const n = parseInt(e.key);
            if (n >= 1 && n <= 7) {
                document.getElementsByClassName("c-button")[n - 1].click();
            }
        });
    });

    return <div className="center">
        <Metadata title="School Websites" description="A list of buttons that link to useful school websites for Hanzehogeschool Groningen. Mostly used by me and my friends to quickly get to the websites we use often." />
        <h1>School Websites</h1>
        <div className="c-buttons">
            <CButton href="https://digirooster.hanze.nl/" color="red" icon={faCalendarAlt} text="Digirooster" />
            <CButton href="https://www.hanze.nl/nld/voorzieningen/voorzieningen/hanzemediatheek" color="orange" icon={faBook} text="Mediatheek" />
            <CButton href="https://www.hanze.nl/" color="yellow" icon={faHome} text="Hanze.nl" />
            <CButton href="https://blackboard.hanze.nl/ultra/institution-page" color="green" icon={faChalkboard} text="Blackboard" />
            <CButton href="https://hanze.osiris-student.nl/" color="blue" icon={faPoll} text="Osiris" />
            <CButton href="https://www.gradescope.com/login" color="purple" icon={faGraduationCap} text="GradeScope" />
            <CButton href="https://www.sv-realtime.nl/home" color="gray" icon={faClock} text="RealTime" />
        </div>
    </div>
}
