import React, { useEffect, useRef, useState } from 'react';

const Terminal: React.FC = () => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [connected, setConnected] = useState(false);
  const terminalRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const ws = new WebSocket(`ws://${location.host}/api/ws/terminal`);

    ws.binaryType = 'arraybuffer'; // Important for correct binary decoding

    ws.onopen = () => {
      console.log('WebSocket connected');
      setConnected(true);
      setSocket(ws);
    };

    ws.onmessage = (event) => {
      const terminal = terminalRef.current;
      if (!terminal) return;

      let text = '';
      if (event.data instanceof ArrayBuffer) {
        text = new TextDecoder().decode(new Uint8Array(event.data));
      } else if (event.data instanceof Blob) {
        event.data.arrayBuffer().then((buffer) => {
          const decoded = new TextDecoder().decode(new Uint8Array(buffer));
          terminal.innerText += decoded;
          terminal.scrollTop = terminal.scrollHeight;
        });
        return;
      } else if (typeof event.data === 'string') {
        text = event.data;
      }

      terminal.innerText += text;
      terminal.scrollTop = terminal.scrollHeight;
    };

    ws.onclose = () => {
      console.log('WebSocket closed');
      setConnected(false);
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendCommand = () => {
    if (socket && socket.readyState === WebSocket.OPEN && inputRef.current) {
      const command = inputRef.current.value + '\n';
      socket.send(command);
      inputRef.current.value = '';
    }
  };

  return (
    <div className="p-4">
      <div
        ref={terminalRef}
        className="bg-black text-green-400 font-mono p-4 rounded-md h-96 overflow-y-auto whitespace-pre-wrap"
        style={{ whiteSpace: 'pre-wrap' }}
      />
      <div className="mt-2 flex">
        <input
          ref={inputRef}
          type="text"
          placeholder="Enter command"
          className="flex-1 px-3 py-2 border rounded-l-md focus:outline-none"
          onKeyDown={(e) => {
            if (e.key === 'Enter') sendCommand();
          }}
        />
        <button
          onClick={sendCommand}
          className="px-4 py-2 bg-blue-600 text-white rounded-r-md"
        >
          Send
        </button>
      </div>
    </div>
  );
};

export default Terminal;
