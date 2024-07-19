import { useAccount } from "wagmi";
import ConnectWallet from "./components/wallet/ConnectWallet";

function App() {
  const { address, isConnecting, isReconnecting } = useAccount();

  return (
    <div className="w-screen h-screen bg-gray-800">
      <div className="w-full h-full flex flex-col items-center justify-center">
        <ConnectWallet />
        <div className="text-sm text-white mt-8">
          {isConnecting || isReconnecting
            ? "loading ..."
            : `Welcome ${address}`}
        </div>
      </div>
    </div>
  );
}

export default App;
