import { useEffect, useState } from 'react';

export const AnimatedLogo = () => {
  const [text, setText] = useState('CosmiC');
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    const animate = async () => {
      if (isAnimating) return;
      setIsAnimating(true);
      const textWithoutLastC = 'Cosmi';

      await new Promise(resolve => setTimeout(resolve, 2000));

      for (let i = textWithoutLastC.length; i >= 1; i--) {
        setText(textWithoutLastC.slice(0, i) + 'C');
        await new Promise(resolve => setTimeout(resolve, 200));
      }

      for (let i = 0; i < textWithoutLastC.length; i++) {
        setText(textWithoutLastC.slice(0, i + 1) + 'C');
        await new Promise(resolve => setTimeout(resolve, 200));
      }

      await new Promise(resolve => setTimeout(resolve, 2000));

      setIsAnimating(false);
    };

    const interval = setInterval(animate, 30000);
    animate();

    return () => clearInterval(interval);
  }, [isAnimating]);

  return (
    <h1 className="text-2xl font-bold text-[#1F6FEB] w-[85px]">
      {text}
    </h1>
  );
};