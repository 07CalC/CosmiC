// src/context/SetupContext.tsx
import React, { createContext, useContext, useEffect, useState } from "react";
import axios from "axios";
import { useGlobal } from "./GlobalContext";

type SetupContextType = {
  needsSetup: boolean | null;
};

const SetupContext = createContext<SetupContextType>({
  needsSetup: null,
});

export const SetupProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [needsSetup, setneedsSetup] = useState<boolean | null>(null);
  const { setGlobalLoading } = useGlobal();

  useEffect(() => {
    axios
      .get("/api/auth/setup")
      .then((res) => {
        setneedsSetup(res.data.needs_setup);
      })
      .catch(() => {
        setneedsSetup(true);
      })
      .finally(() => setGlobalLoading(false));
  }, []);

  return (
    <SetupContext.Provider value={{ needsSetup }}>
      {children}
    </SetupContext.Provider>
  );
};

export const useSetup = () => useContext(SetupContext);
