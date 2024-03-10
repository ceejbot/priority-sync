
# copy fixtures
@fixtures:
	cp -p /Volumes/SkyrimProfiles/2021-04\ simonrim/modlist.txt fixtures/profile_months
	cp -p /Volumes/SkyrimProfiles/2021-04\ simonrim/modlist.txt.2022_02_13_11_22_35 fixtures/profile_2years/modlist.txt
	cp -p /Volumes/SkyrimProfiles/2023-07\ 1.6.640\ mod\ dev/modlist.txt fixtures/profile_week
	cp -p /Volumes/SkyrimProfiles/2023-12\ Minimal\ 1.5.97/modlist.txt fixtures/profile_engine
	cp -p /Volumes/SkyrimProfiles/2024-01\ Smol\ List\ 640/modlist.txt fixtures/profile_newest
	chmod a-x fixtures/*/modlist.txt
