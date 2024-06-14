import { Metadata, ParticlesBG } from "@/components";
import Head from "next/head";
import Image from "next/image";
import { motion } from "framer-motion";


export default function Home() {
  return (
    <>
      <motion.main
        initial={{ opacity: 0, scale: 0.5 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.5 }}
      >
        <Metadata title="Home" description="I'm a Dutch programmer and Ethical Hacker. Interested in cybersecurity and this site contains a blog about it with writeups, tools, and stories. Together with information about me and my projects." />
        <Head>
          <link rel="me" href="https://infosec.exchange/@jorian" />
        </Head>
        <style jsx>{`
        h1, h2 {
          font-family: var(--font-main);
          color: white;
          text-shadow: 0 0 20px black;
        }
        h2 {
          color: var(--foreground-light);
        }
        code {
          color: var(--main-color);
          font-size: 105%;
        }
      `}</style>
        <ParticlesBG />
        <div className="center-transform vw-100">
          <h1>Hello, I am <code>Jorian Woltjer</code></h1>
          <Image className="round-shadow" src="/img/logo.png" width="250" height="250" alt="Logo" />
          <h2 className="my-4">Welcome to my website!</h2>
        </div>
        <a href="/img/88x31.gif" className="center-transform img88x31">
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img src="/img/88x31.gif" alt="88x31" />
        </a>
      </motion.main>
    </>
  )
}