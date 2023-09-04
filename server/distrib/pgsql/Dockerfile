FROM postgres:latest 

# Define environment variables
ENV POSTGRES_USER kratos
ENV POSTGRES_PASSWORD password
ENV POSTGRES_DB nephster

# Expose the PostgreSQL port
EXPOSE 5432

# Add a VOLUME to allow persistence of data 
VOLUME  ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]

# Set environment variables 
# (replace 'mytimezone' with your timezone)
ENV TZ='mytimezone'
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Run PostgreSQL
CMD ["postgres"]
