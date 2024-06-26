import { useRouter } from "next/router";
import { motion } from "framer-motion";

const variants = {
  visible: { x: 0 },
  hidden: (direction) => ({
    x: 15 * direction,
  })
}

function getDirection() {
  if (typeof window === 'undefined') return 0

  let before = new URL(navigation.entries().at(-1).url)
  if (before.href === location.href) {
    before = new URL(navigation.entries().at(-2).url)
  }

  const [beforeSlashes, afterSlashes] = [before, location].map(url => url.pathname.split('/').length - 1)

  const diff = afterSlashes - beforeSlashes
  return diff === 0 ? 0 : diff / Math.abs(diff)
}

export default function TransitionAnimator({ children }) {
  const pathname = useRouter().asPath

  return <motion.main
    key={pathname}
    custom={getDirection()}
    initial="hidden"
    animate="visible"
    variants={variants}
    transition={{ ease: "circOut", duration: 0.1 }}
  >
    {children}
  </motion.main>
}
