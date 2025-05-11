import { FaAirbnb } from 'react-icons/fa';
import NxWelcome from './nx-welcome';

export function App() {
  return (
    <div>
      <NxWelcome title="@cosmi-c/dashboard" />
      <FaAirbnb className='text-4xl self-center'/>
    </div>
  );
}

export default App;
