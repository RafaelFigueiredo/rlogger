import rlogger as logger

logger.debug("testing logger with level 'debug'")
logger.info("testing logger with level 'info'")
logger.warn("testing logger with level 'warn'")
logger.error("testing logger with level 'error'")
logger.critical("testing logger with level 'critical'")


ctx = {
    'version': '0.0.1',
    'environment': 'test'
}
logger.debug("testing logger with level 'debug'", ctx)
logger.info("testing logger with level 'info'", ctx)
logger.warn("testing logger with level 'warn'", ctx)
logger.error("testing logger with level 'error'", ctx)
logger.critical("testing logger with level 'critical'", ctx)

