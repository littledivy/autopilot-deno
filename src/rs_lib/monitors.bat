for /F %%M in ('
    wmic path Win32_PnPEntity where "Service='monitor' and Status='OK'" get DeviceID /VALUE
') do echo Monitors: %%M
