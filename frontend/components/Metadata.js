import Head from 'next/head';
import { useRouter } from 'next/router';
import { HOST } from '@/config';

export default function Metadata({ title, description, img }) {
  const router = useRouter();

  title += " | Jorian Woltjer";
  img = HOST + img;
  const logo = `${HOST}/img/logo.png`;
  const url = `${HOST}${router.asPath}`;
  const domain = new URL(HOST).hostname;

  return <>
    <Head>
      <title>{title}</title>
      <meta name="theme-color" content="#3498db" />
      <link rel="icon" type="image/png" href="/favicon-96x96.png" sizes="96x96" />
      <link rel="icon" type="image/svg+xml" href="favicon.svg" />
      <link rel="shortcut icon" href="/favicon.ico" />
      <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
      <meta name="apple-mobile-web-app-title" content="Jorian" />
      <link rel="manifest" href="/site.webmanifest" />
      {title && <>
        <meta property="og:title" content={title} />
        <meta name="twitter:title" content={title} />
      </>}
      {description && <>
        <meta name="description" content={description} />
        <meta property="og:description" content={description} />
        <meta name="twitter:description" content={description} />
      </>}
      {img ? <>
        <meta property="og:image" content={img} />
        <meta name="twitter:image" content={img} />
        <meta name="twitter:card" content="summary_large_image" />
      </> : <>
        <meta property="og:image" content={logo} />
        <meta name="twitter:image" content={logo} />
        <meta name="twitter:card" content="summary" />
      </>}
      <meta property="og:type" content="website" />
      <meta property="og:url" content={url} />
      <meta name="twitter:url" content={url} />
      <meta property="og:site_name" content={domain} />
      <meta name="twitter:domain" content={domain} />
    </Head>
  </>
}
