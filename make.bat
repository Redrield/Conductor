@ECHO OFF

pushd %~dp0

if "%1" == "" goto all
if "%1" == "all" goto all
if "%1" == "release" goto release

:all
cd ui-react
call yarn
call yarn run build:react
cd..
setlocal
    set RUST_LOG=info && set RUST_BACKTRACE=1 && cargo run
endlocal
goto end

:release
echo "wat go away"
:end


:end
popd