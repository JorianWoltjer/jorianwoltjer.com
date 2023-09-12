import 'bootstrap/dist/css/bootstrap.css'
import '@/styles/fonts.css'
import '@/styles/globals.css'
import '@/styles/react-medium-image-zoom.css'
import { BACKEND_API } from "@/config";
import { useEffect, useState } from "react";
import Head from "next/head";
import Link from "next/link";
import Image from "next/image";
import { config } from '@fortawesome/fontawesome-svg-core'
import '@fortawesome/fontawesome-svg-core/styles.css'
config.autoAddCss = false
import { useRouter } from 'next/router';

function NavbarItem({ href, title }) {
  const router = useRouter();
  const active = router.pathname.startsWith(href) && (href !== "/" || router.pathname === "/");

  return (
    <li className="nav-item">
      <Link className={`nav-link ${active ? 'active' : ''}`} href={href}>{title}</Link>
    </li>
  )
}

function getJavascriptFile(path) {
  const indexes = ["/blog", "/projects"]

  if (path === "/") {
    return "/index.js"
  } else if (indexes.includes(path)) {
    return path + "/index.js"
  } else {
    return path + ".js"
  }
}

export default function App({ Component, pageProps }) {
  const [admin_interface, set_admin_interface] = useState(false);

  const router = useRouter();
  const executingFile = getJavascriptFile(router.pathname);

  useEffect(() => {
    if (document.cookie.includes("admin_interface=true")) {
      set_admin_interface(true);
    }
  }, [])

  const logout = async (e) => {
    e.preventDefault();
    if (confirm("Are you sure you want to logout?")) {
      await fetch(BACKEND_API + "/logout", {
        method: "GET",
        credentials: "same-origin"
      })
      document.cookie = "admin_interface=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";

      set_admin_interface(false);
    }
  }

  return <>
    <Head>
      <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
    </Head>
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark fixed-top">
      <div className="container">
        <Link className="navbar-brand" href="/">
          <Image src="/img/jw.png" alt="JW Logo" width={71} height={60} />
        </Link>
        <button className="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav"
          aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
          <span className="navbar-toggler-icon"></span>
        </button>
        <div className="collapse navbar-collapse" id="navbarNav">
          <ul className="navbar-nav ms-auto">
            {admin_interface &&
              <li className="nav-item">
                <Link className="nav-link gray" id="logout" href="#" onClick={logout}>Logout</Link>
              </li>
            }
            <NavbarItem href="/" title="Home" />
            <NavbarItem href="/blog" title="Blog" />
            <NavbarItem href="/projects" title="Projects" />
            <NavbarItem href="/contact" title="Contact" />
          </ul>
        </div>
      </div>
    </nav>
    <div id="page-content">
      <div className="container">
        <Component {...pageProps} admin_interface={admin_interface} />
      </div>
    </div>
    <footer id="sticky-footer" className="bg-dark text-white-50">
      <div className="container text-center">
        <small>Copyright &copy; {new Date().getFullYear()} Jorian Woltjer. All rights reserved.</small><br />
        <small>Open source on <Link href={`https://github.com/JorianWoltjer/jorianwoltjer.com/blob/main/frontend/pages${executingFile}`} target="_blank" className="white-link">GitHub</Link>{' '}
          (built with <Link className='no-style' href='https://nextjs.org/' target="_blank">NextJS</Link> + <Link className='no-style' href='https://docs.rs/axum/latest/axum/' target="_blank">Axum</Link>)</small>
      </div>
    </footer>
  </>
}
