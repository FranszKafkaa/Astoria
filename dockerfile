#Alterar para rust depois 
FROM ubuntu 


# Só pra lembrar das libs para o ffmpeg
RUN sudo apt update && sudo apt install ffmpeg libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev
