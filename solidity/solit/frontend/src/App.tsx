import ConnectWallet from "./components/wallet/ConnectWallet";

function App() {
  return (
    <div className="w-screen h-screen bg-gray-800">
      <div className="w-full h-full flex items-center justify-center">
        <ConnectWallet />
      </div>
    </div>
  );
}

export default App;
