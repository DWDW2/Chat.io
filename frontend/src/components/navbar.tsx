import { Button } from "./ui/button";

type Props = {};

export function Navbar({}: Props) {
  return (
    <header className="fixed top-4 w-full left-0 right-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 bg-[#d93911] rounded-3xl">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center">
            <a href="#" className="flex items-center">
              <span className="text-white font-bold text-xl tracking-tighter">
                FireChat
              </span>
            </a>
          </div>

          <div className="hidden md:flex space-x-8 font-mono font-semibold">
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              PRICING
            </a>
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              FEATURES
            </a>
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              ENTERPRISE
            </a>
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              BLOG
            </a>
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              FORUM
            </a>
            <a
              href="#"
              className="text-white font-medium hover:text-gray-300 transition-colors"
            >
              CAREERS
            </a>
          </div>

          <div className="flex items-center space-x-4">
            <Button>Login</Button>
          </div>
        </div>
      </div>
    </header>
  );
}
