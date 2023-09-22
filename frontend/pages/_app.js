import { BACKEND_API } from "@/config";
import 'bootstrap/dist/css/bootstrap.css';
import '@/styles/fonts.css';
import '@/styles/globals.css';
import '@/styles/react-medium-image-zoom.css';
import { config } from '@fortawesome/fontawesome-svg-core';
import '@fortawesome/fontawesome-svg-core/styles.css';
import Head from "next/head";
import Image from "next/image";
import Link from "next/link";
import { useRouter } from 'next/router';
import { useEffect, useRef, useState } from "react";
config.autoAddCss = false

function NavbarItem({ href, title, new: _new, ...props }) {
  const router = useRouter();
  const active = router.pathname.startsWith(href) && (href !== "/" || router.pathname === "/");

  return (
    <li className="nav-item">
      {_new && <span className="new-nav-tag">New</span>}
      <Link className={`nav-link ${active ? 'active' : ''}`} href={href} {...props}>{title}</Link>
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
  const [navbarCollapsed, setNavbarCollapsed] = useState(false);
  const [navbarTimeout, setNavbarTimeout] = useState(null);
  const toggleNavbar = useRef(null);

  const router = useRouter();
  const executingFile = getJavascriptFile(router.pathname);

  useEffect(() => {
    if (document.cookie.includes("admin_interface=true")) {
      set_admin_interface(true);
    }

    router.events.on('routeChangeComplete', () => {
      toggleNavbar.current(false);
    });

    const _toggleNavbar = (force) => {
      const navbar = document.getElementById("navbarNav");

      if (force === navbarCollapsed) return;
      const collapsed = force ?? !navbarCollapsed;
      setNavbarCollapsed(collapsed);

      clearTimeout(navbarTimeout);
      if (collapsed) {  // Opening
        navbar.classList.remove("collapse");
        navbar.classList.add("collapsing");
        navbar.style.height = "0"
        navbar.style.height = navbar.scrollHeight + "px"

        setNavbarTimeout(setTimeout(() => {
          navbar.classList.add("collapse");
          navbar.classList.remove("collapsing");
          navbar.classList.add("show");
          navbar.style.height = navbar.scrollHeight + "px"
        }, 350));
      } else {  // Closing
        navbar.classList.remove("collapse");
        navbar.classList.add("collapsing");
        navbar.classList.remove("show");
        navbar.style.height = "0";

        setNavbarTimeout(setTimeout(() => {
          navbar.classList.add("collapse");
          navbar.classList.remove("collapsing");
          navbar.style.height = ""
        }, 350));
      }
    }
    toggleNavbar.current = _toggleNavbar;
  }, [navbarCollapsed, navbarTimeout, router.events])

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
        <button className="custom-toggler navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav"
          aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation" onClick={() => toggleNavbar.current()}>
          <span className="navbar-toggler-icon"></span>
        </button>
        <div className="navbar-collapse collapse" id="navbarNav">
          <ul className="navbar-nav ms-auto">
            {admin_interface &&
              <li className="nav-item">
                <Link className="nav-link gray" id="logout" href="#" onClick={logout}>Logout</Link>
              </li>
            }
            <NavbarItem href="/" title="Home" />
            <NavbarItem href="https://book.jorianwoltjer.com" target="_blank" title=" Book" new={true} />
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
