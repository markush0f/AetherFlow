import { useState } from 'react';
import { Home, Settings, User, ChevronLeft, ChevronRight, LogOut } from 'lucide-react';

export default function Sidebar() {
  const [isOpen, setIsOpen] = useState(true);

  const menuItems = [
    { icon: Home, label: 'Home' },
    { icon: User, label: 'Profile' },
    { icon: Settings, label: 'Settings' },
  ];

  return (
    <aside className={`relative flex flex-col h-screen bg-brand-black border-r border-white/5 transition-all duration-500 ease-[cubic-bezier(0.23,1,0.32,1)] ${isOpen ? 'w-60' : 'w-16'}`}>
      {/* Subtle Toggle */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="absolute -right-3 top-12 flex h-6 w-6 items-center justify-center rounded-sm bg-brand-orange text-brand-black hover:bg-brand-blue shadow-[0_0_15px_rgba(255,107,0,0.2)] transition-all duration-300 z-50 hover:scale-110"
      >
        {isOpen ? <ChevronLeft size={14} /> : <ChevronRight size={14} />}
      </button>

      {/* Brand */}
      <div className={`flex items-center ${isOpen ? 'h-32 px-8' : 'h-20 px-6 justify-center'}`}>
        <div className="relative group transition-all duration-500">
          <img 
            src="/logo.png" 
            alt="Logo" 
            className={`transition-all duration-500 ease-[cubic-bezier(0.23,1,0.32,1)] object-contain ${
              isOpen ? 'h-16 w-auto contrast-110' : 'h-8 w-8 rounded-sm'
            }`}
          />
        </div>
      </div>

      {/* Menu */}
      <nav className="flex-1 px-3 mt-4 space-y-1">
        {menuItems.map((item, index) => (
          <button
            key={index}
            className={`w-full flex items-center p-2.5 rounded-sm transition-all duration-300 hover:bg-brand-orange/5 group ${
              isOpen ? 'justify-start px-3' : 'justify-center'
            }`}
          >
            <item.icon className="h-4 w-4 text-gray-500 group-hover:text-brand-orange transition-colors duration-300" />
            {isOpen && <span className="ml-4 text-[10px] font-light text-gray-400 group-hover:text-white uppercase tracking-widest transition-colors duration-300">{item.label}</span>}
          </button>
        ))}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-white/5">
        <button
          className={`flex items-center w-full transition-all duration-300 group ${isOpen ? 'justify-start space-x-3 px-2' : 'justify-center'}`}
        >
          <div className="h-6 w-6 rounded-full border border-brand-blue group-hover:border-brand-orange flex-shrink-0 transition-colors duration-300" />
          {isOpen && (
            <div className="flex flex-col text-left overflow-hidden">
              <span className="text-[10px] text-gray-500 group-hover:text-white font-medium uppercase tracking-[0.15em] truncate transition-colors duration-300">Markus</span>
            </div>
          )}
        </button>
      </div>
    </aside>
  );
}
