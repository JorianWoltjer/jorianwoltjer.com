import { useRouter } from "next/router";
import { motion } from "framer-motion";
import usePreviousRoute from "@/components/HistoryContext";

const variants = {
  visible: { x: 0 },
  hidden: (direction) => ({
    x: 15 * direction,
  })
}

function getDirection(previous, current) {
  if (!previous) return 0;
  const [previousSlashes, currentSlashes] = [previous, current].map(path => path.split('/').length - 1);

  const diff = currentSlashes - previousSlashes;
  return diff === 0 ? 0 : diff / Math.abs(diff);
}

export default function TransitionAnimator({ children }) {
  const current = useRouter().asPath;
  const previous = usePreviousRoute()?.url;

  return <motion.main
    key={current}
    custom={getDirection(previous, current)}
    initial="hidden"
    animate="visible"
    variants={variants}
    transition={{ ease: "circOut", duration: 0.1 }}
  >
    {children}
  </motion.main>
}
