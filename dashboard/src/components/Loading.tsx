import React from 'react';

const Loading: React.FC = () => {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900">
      <div className="w-16 h-16 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
      <p className="mt-4 text-lg font-semibold text-gray-700 dark:text-gray-300">
        Loading...
      </p>
      <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
        Please wait while we fetch your data
      </p>
    </div>
  );
};

export default Loading;