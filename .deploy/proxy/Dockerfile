FROM nginx:1.23-alpine

COPY ./conf.d/default.conf /etc/nginx/conf.d/default.conf

CMD ["nginx", "-g", "daemon off;"]