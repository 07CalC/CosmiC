import React, { createContext, useContext, useState } from "react";

interface GlobalContextType {
  globalLoading: boolean;
  setGlobalLoading: (loading: boolean) => void;
}

const GlobalContext = createContext<GlobalContextType>({
  globalLoading: false,
  setGlobalLoading: () => {},
});

export const GlobalProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [globalLoading, setGlobalLoading] = useState(true);

  return (
    <GlobalContext.Provider value={{ globalLoading, setGlobalLoading }}>
      {children}
    </GlobalContext.Provider>
  );
};

export const useGlobal = () => useContext(GlobalContext);
