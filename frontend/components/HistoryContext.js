import { createContext, useContext, useRef, useEffect } from 'react';
import { useRouter } from 'next/router';

const HistoryContext = createContext(null);

export const HistoryContextProvider = ({
  children,
}) => {
  const { asPath } = useRouter();

  const ref = useRef(null);

  useEffect(() => {
    ref.current = asPath;
  }, [asPath, ref]);

  const value = {
    url: ref.current,
  };

  return (
    <HistoryContext.Provider value={value}>
      {children}
    </HistoryContext.Provider>
  );
};

export default function usePreviousRoute() {
  return useContext(HistoryContext);
}
