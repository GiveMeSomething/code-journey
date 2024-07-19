import { useConnect } from "wagmi";
import { injected } from "wagmi/connectors";

const ConnectWallet = () => {
  const { connectAsync, isPending } = useConnect();

  const handleConnect = async () => {
    await connectAsync({ connector: injected() });
  };

  return (
    <button
      className="px-4 py-2 rounded-lg border border-white bg-blue-500 hover:bg-blue-600 disabled:to-blue-300 transition-all"
      onClick={handleConnect}
      disabled={isPending}
    >
      <div className="text-white">
        {isPending ? "Loading..." : "Connect Wallet"}
      </div>
    </button>
  );
};

export default ConnectWallet;
