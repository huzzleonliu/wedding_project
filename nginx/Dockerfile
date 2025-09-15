# # building process
# FROM nginx:latest
# WORKDIR /usr/src/app
# run apt-get update &&\
#   apt-get install -y git curl &&\
#   git clone https://github.com/huzzleonliu/wedding_project.git #实际用的时候改成仓库地址
#
#
#
# # runtime process
# FROM nginx:latest
# copy /usr/src/app/wedding_project/nginx/printer_project/dist /usr/share/nginx/html
#
# RUN rm -rf /etc/nginx
# COPY nginx /etc/nginx
#
#
# CMD ["nginx", "-g", "daemon off;"]
#
# building process
FROM nginx:latest
WORKDIR /usr/src/app
run apt-get update &&\
  apt-get install -y git curl &&\
  git clone https://github.com/huzzleonliu/wedding_project.git #实际用的时候改成仓库地址



# # runtime process
# FROM nginx:latest
run cp -rf /usr/src/app/wedding_project/nginx/printer_project/dist /usr/share/nginx/html
#
RUN rm /etc/nginx/nginx.conf
COPY nginx/nginx.conf /etc/nginx/nginx.conf


CMD ["nginx", "-g", "daemon off;"]
