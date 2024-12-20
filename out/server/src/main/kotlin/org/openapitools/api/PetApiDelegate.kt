package org.openapitools.api

import org.openapitools.model.PetGet200Response
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.web.context.request.NativeWebRequest

import java.util.Optional

/**
 * A delegate to be called by the {@link PetApiController}}.
 * Implement this interface with a {@link org.springframework.stereotype.Service} annotated class.
 */
@javax.annotation.Generated(value = ["org.openapitools.codegen.languages.KotlinSpringServerCodegen"], comments = "Generator version: 7.11.0-SNAPSHOT")
interface PetApiDelegate {

    fun getRequest(): Optional<NativeWebRequest> = Optional.empty()

    /**
     * @see PetApi#petGet
     */
    fun petGet(): ResponseEntity<PetGet200Response> {
        getRequest().ifPresent { request ->
            for (mediaType in MediaType.parseMediaTypes(request.getHeader("Accept"))) {
                if (mediaType.isCompatibleWith(MediaType.valueOf("application/json"))) {
                    ApiUtil.setExampleResponse(request, "application/json", "{  \"name\" : \"name\",  \"id\" : 0}")
                    break
                }
            }
        }
        return ResponseEntity(HttpStatus.NOT_IMPLEMENTED)

    }

}
