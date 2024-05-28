import { ErrorPage } from "@/components"

function Error({ statusCode }) {
  return <ErrorPage
    title={statusCode ? `Internal Server Error` : 'Client Error'}
    message={<>There was a {statusCode || 'client'} error loading this page.</>}
  />
}

Error.getInitialProps = ({ res, err }) => {
  const statusCode = res ? res.statusCode : err ? err.statusCode : 404
  return { statusCode }
}

export default Error