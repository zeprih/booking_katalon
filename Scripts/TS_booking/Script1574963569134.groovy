import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper

def jsonSlurper = new JsonSlurper()

'Send request for createToken'
def response = WS.sendRequest(findTestObject('createToken'))

WS.verifyResponseStatusCode(response, 200)

def object = jsonSlurper.parseText(response.getResponseText())

tokenResponses = object.token

println(tokenResponses)

'Send request for createBooking'
def responseCreateBooking = WS.sendRequest(findTestObject('createBooking', [('firstname') : firstname, ('lastname') : lastname
            , ('totalprice') : totalprice, ('depositpaid') : depositpaid, ('checkin') : checkin, ('checkout') : checkout
			, ('additionalneeds') : additionalneeds]))

def objectResponse = jsonSlurper.parseText(responseCreateBooking.getResponseText())

WS.verifyResponseStatusCode(responseCreateBooking, 200)

bookingResponse = objectResponse.bookingid

println(bookingResponse)

'Send request for deletedBooking'
def responseDeleteBooking = WS.sendRequest(findTestObject('deleteBooking', [('id') : bookingResponse, ('token') : tokenResponses]))

WS.verifyResponseStatusCode(responseDeleteBooking, 201)

