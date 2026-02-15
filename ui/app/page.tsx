import Sequence from "@actions/sequence";
import Fallback from "@actions/fallback";
import Takeoff from "@actions/takeoff";
import GotoWaypoint from "@actions/goto_waypoint";
import ReturnHome from "@actions/return_home";
import Land from "@actions/land";
import TakePhoto from "@actions/take_photo";

export default function Home() {
  return (
    <main style={{ padding: "2rem", fontFamily: "system-ui, sans-serif" }}>
      <h1>Behave — Action Components</h1>
      <p style={{ color: "#666", marginBottom: "2rem" }}>
        Each component below is colocated with its Cap&apos;n Proto schema and
        Rust runtime behavior.
      </p>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(160px, 1fr))",
          gap: "1rem",
        }}
      >
        <Sequence />
        <Fallback />
        <Takeoff />
        <GotoWaypoint />
        <ReturnHome />
        <Land />
        <TakePhoto />
      </div>
    </main>
  );
}
