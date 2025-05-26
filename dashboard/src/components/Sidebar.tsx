import { Link, useLocation } from 'react-router-dom';
import { AnimatedLogo } from './AnimatedLogo';
import { MdDashboard } from "react-icons/md";
import { VscGithubProject } from "react-icons/vsc";
import { FaDatabase } from "react-icons/fa";
import { FaRocket } from "react-icons/fa";
import { FaScroll } from "react-icons/fa";
import { IoTerminal } from "react-icons/io5";
import { FaUsers } from "react-icons/fa";
import { IoMdSettings } from "react-icons/io";
import { useAuth } from '../context/authContext';
import { FaUser } from "react-icons/fa";

const routes = [
  { path: '/', label: 'Dashboard', icon: <MdDashboard className="text-2xl" /> },
  { path: '/projects', label: 'Projects', icon: <VscGithubProject className="text-2xl" /> },
  { path: '/databases', label: 'Databases', icon: <FaDatabase className="text-2xl" /> },
  { path: '/deployments', label: 'Deployments', icon: <FaRocket className='text-2xl' /> },
  { path: '/logs', label: 'Logs', icon: <FaScroll className='text-2xl' /> },
  { path: '/terminal', label: 'Terminal', icon: <IoTerminal className='text-2xl' /> },
  { path: '/users', label: 'Users', icon: <FaUsers className='text-2xl' /> },
  { path: '/settings', label: 'Settings', icon: <IoMdSettings className='text-2xl' /> },
];

export const Sidebar = () => {
  const location = useLocation();
  const { user } = useAuth();
  return (
    <div className="h-screen w-64 bg-[#161B22] border-r border-[#30363D] fixed left-0 top-0">
      <div className="p-4 border-b border-[#30363D] flex gap-x-5 items-center">
        <img 
          src='/cosmicLogo.png'
          alt='Logo'
          
          className="w-12 h-16 object-center"
        />
        <AnimatedLogo />
      </div>
      <nav className="p-4">
        {routes.map((route) => (
          <Link
            key={route.path}
            to={route.path}
            className={`flex items-center p-3 mb-2 rounded-md transition-colors
              ${location.pathname === route.path
                ? 'bg-[#1F6FEB] text-[#C9D1D9]'
                : 'text-[#C9D1D9] hover:bg-[#30363D]'
              }`}
          >
            <span className="mr-3">{route.icon}</span>
            {route.label}
          </Link>
        ))}
      </nav>
      <div className="absolute bottom-0 left-0 w-full p-4 flex gap-x-5 border-t items-center border-[#30363D]">
          <FaUser className="text-2xl text-[#C9D1D9]" />
          <span className="text-[#C9D1D9] text-2xl">{user?.username || 'Guest'}</span>
        </div>

    </div>
  );
};