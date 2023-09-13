import { Metadata, Project } from "@/components";
import { BACKEND } from "@/config";

export default function Projects({ projects }) {
    return <>
        <Metadata title="Projects" description="Information about a few projects I made, with images and links to the projects. From websites and coding, to video editing, and utilities. These are some projects I have put a lot of time into." />
        <h1>Projects</h1>
        <div className="row">
            <Project title="This website!" img="this_site.jpg" href="" category="coding"
                text='To learn PHP and have a portfolio in the process, I made this site. It started out 
                        with simple static content, but expanded to a full-blown blog with a custom CMS.
                        Later I rewrote it entirely in NextJS and Rust (using Axum) to practice and improve performance.
                        The whole site and its history are open-source and viewable on 
                        <a href="https://github.com/JorianWoltjer/jorianwoltjer.com" target="_blank">GitHub</a>.'/>
            {projects.map(project => <Project key={project.id} {...project} />)}
        </div>
    </>
}

export async function getStaticProps() {
    const res = await fetch(BACKEND + "/projects");
    const projects = await res.json();

    return {
        props: {
            projects
        }
    }
}
