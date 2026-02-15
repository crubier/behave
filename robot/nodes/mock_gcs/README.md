# Node: Mock GCS

Simulates a ground control station. After a short startup delay, builds a sample "Survey Mission" behavior tree (takeoff, fly to Eiffel Tower, photo, fly to Notre-Dame, photo, return home, land) and sends it as a Cap'n Proto message over UDP to the Communicate node.

Exits after sending the mission.
