import { ParticlesBG } from "@/components";
import Image from "next/image";


export default function Home() {
  return (
    <>
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
      <div className="center">
        <h1>Hello, I am <code>Jorian Woltjer</code></h1>
        <Image className="round-shadow" src="/img/logo.png" width="250" height="250" alt="Logo" />
        <h2 className="my-4">Welcome to my website!</h2>
      </div>
    </>
  )
}