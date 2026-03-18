import { component$ } from "@builder.io/qwik";
import { GothicGame } from "../../types/launcher";

interface SidebarProps {
  selectedGame: GothicGame;
  onGameSelect$: (game: GothicGame) => void;
}

export const Sidebar = component$<SidebarProps>(({ selectedGame, onGameSelect$ }) => {
  const games = [
    { id: GothicGame.Gothic1, name: "Gothic", icon: "⚔️" },
    { id: GothicGame.Gothic2, name: "Gothic II", icon: "🏰" },
    { id: GothicGame.Gothic2NotR, name: "Gothic II: NK", icon: "🏰" },
    { id: GothicGame.ChroniclesOfMyrtana, name: "Archolos", icon: "📜" },
  ];

  return (
    <nav class="sidebar">
      <div class="sidebar-logo">OGL</div>
      <div class="sidebar-nav">
        {games.map((game) => (
          <div
            key={game.id}
            class={["sidebar-item", selectedGame === game.id ? "active" : ""]}
            onClick$={() => onGameSelect$(game.id)}
          >
            <span class="game-icon">{game.icon}</span>
            <span class="game-name">{game.name}</span>
          </div>
        ))}
      </div>
    </nav>
  );
});
