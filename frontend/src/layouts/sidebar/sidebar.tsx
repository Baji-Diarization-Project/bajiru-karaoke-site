import { NavItem } from "./nav-item";

export function Sidebar() {
  return (
    <aside className="flex h-full w-60 shrink-0 flex-col">
      <div className="p-4">
        <span className="text-lg font-bold">Karaoke</span>
      </div>
      <nav className="flex-1 space-y-1 overflow-y-auto px-2 pb-4">
        <NavItem to="/" label="Home" />
        <NavItem to="/search" label="Search" />
        <NavItem to="/artists" label="Artists" />
        <NavItem to="/playlists" label="Playlists" />
      </nav>
    </aside>
  );
}
